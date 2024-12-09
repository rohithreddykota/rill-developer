package testruntime

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	goruntime "runtime"

	"github.com/joho/godotenv"
	"github.com/stretchr/testify/require"
	"github.com/testcontainers/testcontainers-go"
	"github.com/testcontainers/testcontainers-go/modules/clickhouse"
)

// AcquireConnector acquires a test connector by name.
// For a list of available connectors, see the Connectors map below.
func AcquireConnector(t TestingT, name string) map[string]any {
	acquire, ok := Connectors[name]
	require.True(t, ok, "connector not found")
	vars := acquire(t)
	cfg := make(map[string]any, len(vars))
	for k, v := range vars {
		cfg[k] = v
	}
	return cfg
}

// ConnectorAcquireFunc is a function that acquires a connector for a test.
// It should return a map of config keys suitable for passing to drivers.Open.
type ConnectorAcquireFunc func(t TestingT) (vars map[string]string)

// Connectors is a map of available connectors for use in tests.
// When acquiring a connector, it will only be cleaned up when the test has completed.
// You should avoid acquiring the same connector multiple times in the same test.
//
// Test connectors can either be implemented as:
// - Services embedded in the current process
// - Services started as ephemeral testcontainers
// - Real external services configured for use in tests with credentials provided in the root .env file with the prefix RILL_RUNTIME_TEST_.
var Connectors = map[string]ConnectorAcquireFunc{
	// clickhouse starts a ClickHouse test container with no tables initialized.
	"clickhouse": func(t TestingT) map[string]string {
		_, currentFile, _, _ := goruntime.Caller(0)
		testdataPath := filepath.Join(currentFile, "..", "testdata")

		ctx := context.Background()
		clickHouseContainer, err := clickhouse.Run(
			ctx,
			"clickhouse/clickhouse-server:24.6.2.17",
			clickhouse.WithUsername("clickhouse"),
			clickhouse.WithPassword("clickhouse"),
			clickhouse.WithConfigFile(filepath.Join(testdataPath, "clickhouse-config.xml")),
			testcontainers.CustomizeRequestOption(func(req *testcontainers.GenericContainerRequest) error {
				cf := testcontainers.ContainerFile{
					HostFilePath:      filepath.Join(testdataPath, "users.xml"),
					ContainerFilePath: "/etc/clickhouse-server/users.xml",
					FileMode:          0o755,
				}
				req.Files = append(req.Files, cf)
				return nil
			}),
		)
		require.NoError(t, err)

		t.Cleanup(func() {
			err := clickHouseContainer.Terminate(ctx)
			require.NoError(t, err)
		})

		host, err := clickHouseContainer.Host(ctx)
		require.NoError(t, err)
		port, err := clickHouseContainer.MappedPort(ctx, "9000/tcp")
		require.NoError(t, err)

		dsn := fmt.Sprintf("clickhouse://clickhouse:clickhouse@%v:%v", host, port.Port())
		return map[string]string{"dsn": dsn}
	},

	// druid connects to a real Druid cluster using the connection string in RILL_RUNTIME_DRUID_TEST_DSN.
	// This usually uses the master.in cluster.
	"druid": func(t TestingT) map[string]string {
		// Load .env file at the repo root (if any)
		_, currentFile, _, _ := goruntime.Caller(0)
		envPath := filepath.Join(currentFile, "..", "..", "..", ".env")
		_, err := os.Stat(envPath)
		if err == nil {
			require.NoError(t, godotenv.Load(envPath))
		}

		dsn := os.Getenv("RILL_RUNTIME_DRUID_TEST_DSN")
		require.NotEmpty(t, dsn, "Druid test DSN not configured")
		return map[string]string{"dsn": dsn}
	},
}
