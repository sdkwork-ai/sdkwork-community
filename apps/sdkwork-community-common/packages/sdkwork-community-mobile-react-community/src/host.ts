/**
 * Host integration surface for the community mobile UI.
 *
 * Non-mobile hosts (for example the Cloud Router PC workspace) need the
 * SDK-backed service and the runtime ports without importing the mobile React
 * pages or components. This barrel exposes exactly that boundary; it stays a
 * public package subpath (`@sdkwork/community-mobile-react-community/host`).
 */
export * from "./services/CommunityService";
export * from "./services/communityAuthSessionPort";
export * from "./services/communityMediaRuntimePort";
export * from "./services/communityOrderRuntime";
export * from "./services/communityRuntimePort";
export * from "./types";
