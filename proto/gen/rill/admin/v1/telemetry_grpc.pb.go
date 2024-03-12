// Code generated by protoc-gen-go-grpc. DO NOT EDIT.
// versions:
// - protoc-gen-go-grpc v1.3.0
// - protoc             (unknown)
// source: rill/admin/v1/telemetry.proto

package adminv1

import (
	context "context"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
)

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
// Requires gRPC-Go v1.32.0 or later.
const _ = grpc.SupportPackageIsVersion7

const (
	TelemetryService_RecordEvents_FullMethodName = "/rill.admin.v1.TelemetryService/RecordEvents"
)

// TelemetryServiceClient is the client API for TelemetryService service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type TelemetryServiceClient interface {
	// RecordEvents sends a batch of telemetry events.
	// The events must conform to the schema described in rill/runtime/pkg/activity/README.md.
	RecordEvents(ctx context.Context, in *RecordEventsRequest, opts ...grpc.CallOption) (*RecordEventsResponse, error)
}

type telemetryServiceClient struct {
	cc grpc.ClientConnInterface
}

func NewTelemetryServiceClient(cc grpc.ClientConnInterface) TelemetryServiceClient {
	return &telemetryServiceClient{cc}
}

func (c *telemetryServiceClient) RecordEvents(ctx context.Context, in *RecordEventsRequest, opts ...grpc.CallOption) (*RecordEventsResponse, error) {
	out := new(RecordEventsResponse)
	err := c.cc.Invoke(ctx, TelemetryService_RecordEvents_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// TelemetryServiceServer is the server API for TelemetryService service.
// All implementations must embed UnimplementedTelemetryServiceServer
// for forward compatibility
type TelemetryServiceServer interface {
	// RecordEvents sends a batch of telemetry events.
	// The events must conform to the schema described in rill/runtime/pkg/activity/README.md.
	RecordEvents(context.Context, *RecordEventsRequest) (*RecordEventsResponse, error)
	mustEmbedUnimplementedTelemetryServiceServer()
}

// UnimplementedTelemetryServiceServer must be embedded to have forward compatible implementations.
type UnimplementedTelemetryServiceServer struct {
}

func (UnimplementedTelemetryServiceServer) RecordEvents(context.Context, *RecordEventsRequest) (*RecordEventsResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method RecordEvents not implemented")
}
func (UnimplementedTelemetryServiceServer) mustEmbedUnimplementedTelemetryServiceServer() {}

// UnsafeTelemetryServiceServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to TelemetryServiceServer will
// result in compilation errors.
type UnsafeTelemetryServiceServer interface {
	mustEmbedUnimplementedTelemetryServiceServer()
}

func RegisterTelemetryServiceServer(s grpc.ServiceRegistrar, srv TelemetryServiceServer) {
	s.RegisterService(&TelemetryService_ServiceDesc, srv)
}

func _TelemetryService_RecordEvents_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(RecordEventsRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(TelemetryServiceServer).RecordEvents(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: TelemetryService_RecordEvents_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(TelemetryServiceServer).RecordEvents(ctx, req.(*RecordEventsRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// TelemetryService_ServiceDesc is the grpc.ServiceDesc for TelemetryService service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var TelemetryService_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "rill.admin.v1.TelemetryService",
	HandlerType: (*TelemetryServiceServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "RecordEvents",
			Handler:    _TelemetryService_RecordEvents_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "rill/admin/v1/telemetry.proto",
}