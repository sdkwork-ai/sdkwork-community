/**
 * Host-injectable media runtime port for the community UI.
 *
 * Post images are uploaded by the host through the platform drive uploader
 * (the standard IM H5 pattern: `uploader.uploadImage` → `drive://` URL), then
 * stored on the backend entry as media URLs. Hosts without a drive client
 * leave the port unconfigured; the UI hides the image picker instead of
 * fabricating local-only media.
 */

export interface CommunityMediaRuntimePort {
  /** Uploads images and returns their backend-addressable URLs. */
  uploadImages(files: File[]): Promise<string[]>;
  /**
   * Transient display URL for a stored media URI (drive:// URIs need a
   * bounded host-side read; http(s) URLs resolve to themselves). Optional:
   * hosts that never store drive-backed media may omit it.
   */
  resolveDisplayUrl?(uri: string): Promise<string | null>;
}

let mediaRuntimePort: CommunityMediaRuntimePort | null = null;

export function configureCommunityMediaRuntimePort(port: CommunityMediaRuntimePort): void {
  mediaRuntimePort = port;
}

export function resetCommunityMediaRuntimePort(): void {
  mediaRuntimePort = null;
}

export function isCommunityMediaRuntimeConfigured(): boolean {
  return mediaRuntimePort !== null;
}

export function getCommunityMediaRuntime(): CommunityMediaRuntimePort {
  if (!mediaRuntimePort) {
    throw new Error(
      "community media runtime port is not configured: the host must inject a drive-backed " +
        "image uploader via configureCommunityMediaRuntimePort before publishing media posts",
    );
  }
  return mediaRuntimePort;
}
