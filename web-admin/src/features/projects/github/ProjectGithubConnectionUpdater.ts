import {
  createAdminServiceConnectProjectToGithub,
  getAdminServiceGetGithubUserStatusQueryKey,
  getAdminServiceGetProjectQueryKey,
  type ListGithubUserReposResponseRepo,
} from "@rilldata/web-admin/client";
import { extractGithubConnectError } from "@rilldata/web-admin/features/projects/github/github-errors";
import { queryClient } from "@rilldata/web-common/lib/svelte-query/globalQueryClient";
import { behaviourEvent } from "@rilldata/web-common/metrics/initMetrics";
import { BehaviourEventAction } from "@rilldata/web-common/metrics/service/BehaviourEventTypes";
import { invalidateRuntimeQueries } from "@rilldata/web-common/runtime-client/invalidation";
import { get, writable } from "svelte/store";

/**
 * Wrapper around ConnectProjectToGithub RPC with some extra state moved out of components
 */
export class ProjectGithubConnectionUpdater {
  public readonly showOverwriteConfirmation = writable(false);
  public readonly connectToGithubMutation =
    createAdminServiceConnectProjectToGithub();

  public readonly githubUrl = writable("");
  public readonly subpath = writable("");
  public readonly branch = writable("");

  private readonly isConnected: boolean;
  private defaultBranch = "";

  public constructor(
    private readonly organization: string,
    private readonly project: string,
    githubUrl: string,
    subpath: string,
    branch: string,
  ) {
    this.githubUrl.set(githubUrl);
    this.subpath.set(subpath);
    this.branch.set(branch);
    this.isConnected = !!githubUrl;
  }

  public onSelectedRepoChange(repo: ListGithubUserReposResponseRepo) {
    this.subpath.set("");
    this.branch.set(repo.defaultBranch ?? "");
    this.defaultBranch = repo.defaultBranch ?? "";
  }

  public async update({
    instanceId,
    force,
  }: {
    instanceId: string;
    force: boolean;
  }) {
    const githubUrl = get(this.githubUrl);
    const subpath = get(this.subpath);
    const branch = get(this.branch);
    const hasSubpath = !!subpath;
    const hasNonDefaultBranch = branch !== this.defaultBranch;

    try {
      await get(this.connectToGithubMutation).mutateAsync({
        organization: this.organization,
        project: this.project,
        data: {
          repo: githubUrl,
          subpath,
          branch,
          force,
        },
      });

      behaviourEvent?.fireGithubIntentEvent(
        BehaviourEventAction.GithubConnectSuccess,
        {
          is_overwrite: force,
          has_subpath: hasSubpath,
          has_non_default_branch: hasNonDefaultBranch,
          is_fresh_connection: this.isConnected,
        },
      );

      void queryClient.refetchQueries(
        getAdminServiceGetProjectQueryKey(this.organization, this.project),
        {
          // avoid refetching createAdminServiceGetProjectWithBearerToken
          exact: true,
        },
      );
      void queryClient.refetchQueries(
        getAdminServiceGetGithubUserStatusQueryKey(),
      );
      void invalidateRuntimeQueries(queryClient, instanceId);
    } catch (e) {
      const err = extractGithubConnectError(e);
      if (!force && err.notEmpty) {
        behaviourEvent?.fireGithubIntentEvent(
          BehaviourEventAction.GithubConnectOverwritePrompt,
          {
            has_subpath: hasSubpath,
            has_non_default_branch: hasNonDefaultBranch,
            is_fresh_connection: this.isConnected,
          },
        );
        this.showOverwriteConfirmation.set(true);
        return false;
      } else {
        behaviourEvent?.fireGithubIntentEvent(
          BehaviourEventAction.GithubConnectFailure,
          {
            is_overwrite: force,
            has_subpath: hasSubpath,
            has_non_default_branch: hasNonDefaultBranch,
            is_fresh_connection: this.isConnected,
            failure_error: err.message,
          },
        );
        throw e;
      }
    }

    return true;
  }
}