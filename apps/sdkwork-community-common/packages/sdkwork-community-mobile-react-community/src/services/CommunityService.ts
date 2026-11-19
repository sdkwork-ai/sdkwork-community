import type {
  SdkworkCommunityCategory,
  SdkworkCommunityComment,
  SdkworkCommunityEntry,
  SdkworkCommunityGroup,
  SdkworkCommunityMember as SdkworkCommunityMemberRecord,
  SdkworkCommunityMembershipTier,
} from "@sdkwork/community-contracts";
import type { SdkworkCommunityAppSdkPort } from "@sdkwork/community-sdk-ports";
import type { FeedItem, SdkworkFeedsClient } from "@sdkwork/feeds-sdk";
import {
  getCommunityFeedsPort,
  getCommunityRuntimePort,
  isCommunityFeedsPortConfigured,
} from "./communityRuntimePort";
import {
  getCommunityMediaRuntime,
  isCommunityMediaRuntimeConfigured,
} from "./communityMediaRuntimePort";
import type { Community, CommunityGroup, CommunityMember, MembershipTier, Post, PostComment, Resource } from "../types";

/**
 * Community (圈子) service facade for the mobile React UI.
 *
 * Every read and write delegates to the configured community App SDK port
 * (generated SDK backed by the backend service). There is deliberately no
 * client-local data: circles, posts, comments, media, resources, groups and
 * memberships all come from the backend, and all entity ids are minted
 * server-side (snowflake).
 */

// Session-local liked post ids (reactions are per-viewer; the backend keeps
// the authoritative reaction count, the set only tracks the viewer's state).
const LIKED_POST_IDS = new Set<string>();

// Entries the current user just created are inserted as drafts and are not in
// the approved public feed until moderation approves them. Keep the author's
// own pending entries visible in their session (with a pending badge); the
// server remains the authority and the id de-duplicates once approved.
const PENDING_POSTS = new Map<string, Post[]>();
const PENDING_RESOURCES = new Map<string, Resource[]>();

function rememberPendingPost(communityId: string, post: Post): void {
  if (post.reviewState === "approved") {
    return;
  }
  const current = PENDING_POSTS.get(communityId) ?? [];
  PENDING_POSTS.set(communityId, [post, ...current.filter((item) => item.id !== post.id)]);
}

function rememberPendingResource(communityId: string, resource: Resource): void {
  if (resource.reviewState === "approved") {
    return;
  }
  const current = PENDING_RESOURCES.get(communityId) ?? [];
  PENDING_RESOURCES.set(communityId, [resource, ...current.filter((item) => item.id !== resource.id)]);
}

function mergePendingPosts(communityId: string, posts: Post[], tag?: string): Post[] {
  const pending = (PENDING_POSTS.get(communityId) ?? []).filter(
    (post) => !tag || post.tags?.includes(tag),
  );
  if (pending.length === 0) {
    return posts;
  }
  const ids = new Set(posts.map((post) => post.id));
  return [...pending.filter((post) => !ids.has(post.id)), ...posts];
}

function mergePendingResources(communityId: string, resources: Resource[]): Resource[] {
  const pending = PENDING_RESOURCES.get(communityId) ?? [];
  if (pending.length === 0) {
    return resources;
  }
  const ids = new Set(resources.map((resource) => resource.id));
  return [...pending.filter((resource) => !ids.has(resource.id)), ...resources];
}

function port(): SdkworkCommunityAppSdkPort {
  return getCommunityRuntimePort();
}

/** Standard feeds stream key for one circle's posts (all kinds). */
function circlePostsStreamKey(communityId: string): string {
  return `community-${communityId}`;
}

/** Standard feeds stream key for one circle's resources (kind=resource). */
function circleResourcesStreamKey(communityId: string): string {
  return `community-${communityId}-resources`;
}

function payloadKind(item: FeedItem): string {
  const kind = item.payload?.kind;
  return typeof kind === "string" ? kind : "";
}

function toReadableFeedText(item: FeedItem): string {
  return (item.excerpt ?? item.title ?? "").trim();
}

function mapFeedItemToPost(item: FeedItem): Post {
  const payload = item.payload as Record<string, unknown> | undefined;
  const media = Array.isArray(payload?.media)
    ? payload.media.filter((entry): entry is string => typeof entry === "string")
    : Array.isArray(payload?.images)
      ? payload.images.filter((entry): entry is string => typeof entry === "string")
      : undefined;
  const body = typeof payload?.body === "string" ? payload.body : undefined;
  return {
    id: item.id,
    communityId: item.streamKey?.replace(/^community-/, "").replace(/-resources$/, "") ?? "",
    authorId: item.author?.id ?? "",
    authorName: item.author?.name ?? "",
    authorAvatar: item.author?.avatarUrl ?? "",
    content: body?.trim() || toReadableFeedText(item),
    images: media && media.length > 0 ? media : undefined,
    createdAt: String(item.publishedAt ?? item.createdAt ?? ""),
    likes: item.reactionCount ?? 0,
    comments: item.commentCount ?? 0,
    isLiked: LIKED_POST_IDS.has(item.id),
    reviewState: "approved",
  };
}

function mapFeedItemToResource(item: FeedItem): Resource {
  const payload = item.payload as Record<string, unknown> | undefined;
  const firstTag = typeof payload?.tag === "string" ? payload.tag : "";
  const payloadUrl =
    typeof payload?.url === "string"
      ? payload.url
      : Array.isArray(payload?.media)
        ? payload.media.find((entry): entry is string => typeof entry === "string")
        : undefined;
  return {
    id: item.id,
    communityId: item.streamKey?.replace(/^community-/, "").replace(/-resources$/, "") ?? "",
    title: item.title,
    type: firstTag || "link",
    url: payloadUrl ?? item.coverUrl ?? "#",
    createdAt: String(item.publishedAt ?? item.createdAt ?? ""),
    uploadedBy: item.author?.name ?? "",
    reviewState: "approved",
  };
}

/**
 * Resolves one stored media URI through the host media port.
 *
 * Drive references are not browser-loadable; hosts with a media runtime
 * resolve them to a short-lived URL. Non-drive values and hosts without a
 * resolver keep the stored value.
 */
async function resolveMediaUri(uri: string | undefined): Promise<string> {
  if (!uri || !uri.startsWith("drive://") || !isCommunityMediaRuntimeConfigured()) {
    return uri ?? "";
  }
  const resolve = getCommunityMediaRuntime().resolveDisplayUrl;
  if (!resolve) {
    return uri;
  }
  return (await resolve(uri)) ?? uri;
}

/** Resolves circle avatar/cover media for display. */
async function resolveCommunityMedia(community: Community): Promise<Community> {  const [avatar, coverImage] = await Promise.all([
    resolveMediaUri(community.avatar),
    resolveMediaUri(community.coverImage),
  ]);
  return {
    ...community,
    avatar: avatar || undefined,
    coverImage,
    ...(community.avatar?.startsWith("drive://") ? { rawAvatar: community.avatar } : {}),
    ...(community.coverImage?.startsWith("drive://") ? { rawCoverImage: community.coverImage } : {}),
  };
}

/** Loads a post's comments without failing the whole feed on one error. */
async function loadComments(entryId: string): Promise<PostComment[]> {
  try {
    const comments = await port().community.comments.list(entryId);
    return comments.map(mapCommentToPostComment);
  } catch {
    return [];
  }
}

/**
 * Resolves stored drive:// media into transient display URLs.
 *
 * Post media is persisted as a Drive reference, never as a delivery URL. A
 * browser cannot load `drive://...` directly, so the host media port resolves
 * each reference before the post renders. Hosts without a media port keep the
 * stored value (their composer never produced drive media either).
 */
async function resolvePostImages(post: Post): Promise<Post> {
  if (!post.images?.length || !isCommunityMediaRuntimeConfigured()) {
    return post;
  }
  const resolve = getCommunityMediaRuntime().resolveDisplayUrl;
  if (!resolve) {
    return post;
  }
  const images = await Promise.all(
    post.images.map(async (uri) => (uri.startsWith("drive://") ? (await resolve(uri)) ?? uri : uri)),
  );
  return { ...post, images };
}

/** Lightweight fallback path for hosts whose app-api feed is unavailable. */
async function loadPostsFromFeeds(
  communityId: string,
  tag: string | undefined,
): Promise<Post[]> {
  if (!isCommunityFeedsPortConfigured()) {
    return [];
  }
  // The feeds stream has no tag parameter; a tagged tab is an app-api read.
  if (tag) {
    return [];
  }
  const items = await listCircleFeedItems(
    getCommunityFeedsPort(),
    circlePostsStreamKey(communityId),
  );
  const posts = items
    .filter((item) => payloadKind(item) !== "resource")
    .map(mapFeedItemToPost);
  return Promise.all(
    posts.map(async (post) => {
      const withComments = { ...post, commentsList: await loadComments(post.id) };
      return resolvePostImages(withComments);
    }),
  );
}

/** Reads one circle feed page through the standard feeds stream client. */
async function listCircleFeedItems(
  feeds: SdkworkFeedsClient,
  streamKey: string,
): Promise<FeedItem[]> {
  const page = await feeds.feeds.streams.items.list(streamKey, { pageSize: 100 });
  return page.items as unknown as FeedItem[];
}

function mapCategoryToCommunity(category: SdkworkCommunityCategory, isJoined: boolean): Community {
  return {
    id: category.id,
    name: category.title,
    description: category.description ?? "",
    coverImage: category.coverImage ?? "",
    avatar: category.avatar,
    ownerId: category.ownerId,
    memberCount: category.memberCount ?? 0,
    memberLimit: category.memberLimit,
    postCount: category.postCount ?? 0,
    revenueTarget: category.revenueTarget,
    revenueRaised: category.revenueRaised ?? 0,
    tags: category.tags ? [...category.tags] : [],
    tabs: category.tabs ? [...category.tabs] : undefined,
    isJoined,
    isPaid: category.isPaid,
    price: category.price,
  };
}

function mapCommentToPostComment(comment: SdkworkCommunityComment): PostComment {
  return {
    id: comment.id,
    authorName: comment.author.name,
    content: comment.body,
    createdAt: String(comment.createdAt),
  };
}

/**
 * Human-readable fields that product surfaces may embed in JSON-structured
 * entry bodies (e.g. the Agents 灵感广场 activity cards). The feed must never
 * dump the raw JSON document into a post.
 */
const JSON_BODY_TEXT_FIELDS = [
  "background",
  "desc",
  "description",
  "summary",
  "timeRange",
  "status",
  "tag",
] as const;

/** True when the entry body is a serialized JSON document (object or array). */
function isJsonDocument(value: string): boolean {
  const trimmed = value.trim();
  if (!trimmed.startsWith("{") && !trimmed.startsWith("[")) {
    return false;
  }
  try {
    return typeof JSON.parse(trimmed) === "object";
  } catch {
    return false;
  }
}

/**
 * Renders an entry body as readable post text. Plain markdown passes through;
 * JSON-structured bodies (used as structured payloads by other surfaces) are
 * reduced to their human-readable text fields, falling back to excerpt/title.
 */
function toReadableEntryText(body: string | undefined, excerpt?: string, title?: string): string {
  if (!body) {
    return excerpt?.trim() || title?.trim() || "";
  }
  if (!isJsonDocument(body)) {
    return body;
  }
  try {
    const record = JSON.parse(body.trim()) as Record<string, unknown>;
    const parts = JSON_BODY_TEXT_FIELDS.map((key) => record[key])
      .filter((value): value is string => typeof value === "string" && value.trim().length > 0)
      .map((value) => value.trim());
    if (parts.length > 0) {
      return parts.join("\n\n");
    }
  } catch {
    // Fall through to the excerpt/title fallback below.
  }
  return excerpt?.trim() || title?.trim() || "";
}

function mapEntryToPost(entry: SdkworkCommunityEntry, commentsList?: PostComment[]): Post {
  return {
    id: entry.id,
    communityId: entry.categoryId,
    authorId: entry.author.id,
    authorName: entry.author.name,
    authorAvatar: entry.author.avatar?.publicUrl ?? "",
    content: toReadableEntryText(entry.body, entry.excerpt, entry.title),
    images: entry.media ? [...entry.media] : undefined,
    createdAt: String(entry.publishedAt ?? entry.lastActivityAt ?? ""),
    likes: entry.stats.reactionCount ?? 0,
    comments: commentsList?.length ?? entry.stats.commentCount ?? 0,
    commentsList,
    isLiked: LIKED_POST_IDS.has(entry.id),
    reviewState: entry.reviewState,
    tags: entry.tags ? [...entry.tags] : undefined,
  };
}

/** Maps a backend entry of kind "resource" to the resources-tab view. */
function mapEntryToResource(entry: SdkworkCommunityEntry): Resource {
  return {
    id: entry.id,
    communityId: entry.categoryId,
    title: entry.title,
    // Entries carry the resource type through their first tag (pdf/doc/...).
    type: entry.tags?.[0] ?? "link",
    url: entry.media?.[0] ?? "#",
    uploadedBy: entry.author.name,
    createdAt: String(entry.publishedAt ?? entry.lastActivityAt ?? ""),
    reviewState: entry.reviewState,
  };
}

function mapGroupToCommunityGroup(group: SdkworkCommunityGroup): CommunityGroup {
  return {
    id: group.id,
    communityId: group.communityId,
    name: group.name,
    platform: group.platform,
    description: group.description,
    memberCount: group.memberCount,
    qrCodeUrl: group.qrCodeUrl ?? group.qrCodes?.[0]?.url,
    qrCodes: group.qrCodes
      ? group.qrCodes.map((item) => ({ url: item.url, description: item.description ?? "" }))
      : undefined,
    createdAt: String(group.createdAt),
  };
}

function mapMemberToCommunityMember(member: SdkworkCommunityMemberRecord): CommunityMember {
  return {
    id: member.id,
    communityId: member.communityId,
    name: member.user.name,
    avatar: member.user.avatar?.publicUrl ?? "",
    role: member.role,
    joinDate: String(member.joinedAt),
    status: member.status,
    bio: member.bio,
    tierId: member.tierId,
    tierName: member.tierName,
    membershipExpiresAt: member.membershipExpiresAt
      ? String(member.membershipExpiresAt)
      : undefined,
  };
}

function mapTierToMembershipTier(tier: SdkworkCommunityMembershipTier): MembershipTier {
  return {
    id: tier.id,
    categoryId: tier.categoryId,
    name: tier.name,
    description: tier.description,
    price: tier.price,
    durationDays: tier.durationDays,
    benefits: [...tier.benefits],
    enabled: tier.enabled,
    sortOrder: tier.sortOrder,
    catalogPackageId: tier.catalogPackageId,
  };
}

/** True when the error indicates the circle member limit was reached. */
export function isMemberLimitError(error: unknown): boolean {
  const message = error instanceof Error ? error.message : String(error);
  return message.includes("limit") || message.includes("满");
}

/** True when the circle's funding target was reached and purchases are closed. */
export function isRevenueTargetError(error: unknown): boolean {
  const message = error instanceof Error ? error.message : String(error);
  return message.includes("revenue target") || message.includes("融资") || message.includes("目标已达成");
}

function truncate(value: string, length: number): string {
  const normalized = value.trim();
  return normalized.length <= length ? normalized : `${normalized.slice(0, length)}…`;
}

export const CommunityService = {
  async getMembersByCommunity(communityId: string): Promise<CommunityMember[]> {
    const members = await port().community.members.list(communityId);
    return members.map(mapMemberToCommunityMember);
  },

  async updateMemberRole(
    communityId: string,
    memberId: string,
    role: CommunityMember["role"],
  ): Promise<void> {
    await port().community.members.updateRole(communityId, memberId, role);
  },

  async updateMemberStatus(
    communityId: string,
    memberId: string,
    status: CommunityMember["status"],
  ): Promise<void> {
    await port().community.members.updateStatus(communityId, memberId, status);
  },

  async removeMember(communityId: string, memberId: string): Promise<void> {
    await port().community.members.remove(communityId, memberId);
  },

  async createCommunity(
    community: Omit<Community, "id" | "memberCount" | "postCount" | "isJoined">,
  ): Promise<Community> {
    const category = await port().community.categories.create({
      title: community.name,
      description: community.description,
      avatar: community.avatar,
      coverImage: community.coverImage,
      isPaid: community.isPaid,
      memberLimit: community.memberLimit,
      price: community.price,
      revenueTarget: community.revenueTarget,
      tags: community.tags,
      tabs: community.tabs,
    });
    return resolveCommunityMedia(mapCategoryToCommunity(category, true));
  },

  async getCommunities(): Promise<Community[]> {
    const categories = await port().community.categories.list();
    // The list endpoint already carries the current user's membership state;
    // no per-circle members.current requests (N+1).
    return Promise.all(
      categories.map(async (category) =>
        resolveCommunityMedia(mapCategoryToCommunity(category, Boolean(category.isJoined))),
      ),
    );
  },

  async getCommunityById(id: string): Promise<Community | undefined> {
    try {
      const category = await port().community.categories.retrieve(id);
      return resolveCommunityMedia(mapCategoryToCommunity(category, Boolean(category.isJoined)));
    } catch {
      // The backend returns not-found for missing or disabled circles.
      return undefined;
    }
  },

  async joinCommunity(id: string): Promise<void> {
    await port().community.members.join(id);
  },

  /** Leaves a circle: resolves the current membership and removes it. */
  async leaveCommunity(communityId: string): Promise<void> {
    const member = await port().community.members.current(communityId);
    if (!member) {
      return;
    }
    await port().community.members.remove(communityId, member.id);
  },

  /** Deletes a circle; the backend requires the owner role. */
  async deleteCommunity(communityId: string): Promise<void> {
    await port().community.categories.remove(communityId);
  },

  /**
   * Lists circle posts.
   *
   * The app-api feed is authoritative and returns the complete entry (body,
   * media, tags, stats). The standard feeds stream is a lightweight projection
   * that intentionally drops the body and media, so it is only a fallback for
   * hosts where the app-api feed is unavailable. Never render the lightweight
   * projection while a complete read is available: posts would silently lose
   * their images and their body beyond the excerpt.
   */
  async getPostsByCommunity(
    communityId: string,
    options: { tag?: string } = {},
  ): Promise<Post[]> {
    const tag = options.tag?.trim();
    try {
      const entries = await port().community.feed.list({
        categoryId: communityId,
        ...(tag ? { tag } : {}),
      });
      // The default 动态 tab shows non-resource entries; resources have their
      // own tab. A tagged tab (news/docs/repos/software) shows every entry
      // carrying that tag, including resource entries.
      const visible = tag ? entries : entries.filter((entry) => entry.kind !== "resource");
      const posts = await Promise.all(
        visible.map(async (entry) =>
          resolvePostImages(mapEntryToPost(entry, await loadComments(entry.id))),
        ),
      );
      return mergePendingPosts(communityId, posts, tag);
    } catch {
      return mergePendingPosts(communityId, await loadPostsFromFeeds(communityId, tag), tag);
    }
  },

  /** Lists entries carrying one circle tab tag (news/docs/repos/software). */
  async getPostsByCommunityTag(communityId: string, tag: string): Promise<Post[]> {
    return this.getPostsByCommunity(communityId, { tag });
  },

  /**
   * Creates a post. Images are uploaded through the host-injected media
   * runtime (drive-backed) and stored on the backend entry as media URLs;
   * the backend mints the post id. Circle tab entries pass their tab tag so
   * the news/docs/repos/software tabs can render them.
   */
  async createPost(
    communityId: string,
    content: string,
    images?: File[],
    options: {
      kind?: SdkworkCommunityEntry["kind"];
      tags?: readonly string[];
      title?: string;
    } = {},
  ): Promise<Post> {
    const media =
      images && images.length > 0 ? await getCommunityMediaRuntime().uploadImages(images) : [];
    const kind = options.kind ?? "discussion";
    const tags = (options.tags ?? []).map((tag) => tag.trim()).filter(Boolean);
    const entry = await port().community.entries.create({
      categoryId: communityId,
      kind,
      title: options.title?.trim() || truncate(content.split("\n")[0] ?? "", 120) || "Untitled",
      excerpt: truncate(content, 240),
      body: content,
      media,
      ...(tags.length > 0 ? { tags } : {}),
    });
    const post = await resolvePostImages(mapEntryToPost(entry));
    rememberPendingPost(communityId, post);
    return post;
  },

  /**
   * Publishes a resource into a circle. The entry keeps the resource type in
   * its first tag and the addressable URL in its media list, which is the
   * shape the resources tab reads.
   */
  async createResource(
    communityId: string,
    resource: {
      title: string;
      url: string;
      type?: string;
      description?: string;
    },
  ): Promise<Resource> {
    const type = resource.type?.trim() || "link";
    const entry = await port().community.entries.create({
      categoryId: communityId,
      kind: "resource",
      title: resource.title.trim(),
      excerpt: resource.description?.trim() || resource.title.trim(),
      body: resource.description?.trim() || resource.title.trim(),
      media: [resource.url.trim()],
      tags: [type, "resource"],
    });
    const mapped = mapEntryToResource(entry);
    rememberPendingResource(communityId, mapped);
    return mapped;
  },

  /** Creates a comment and returns the backend-minted comment (id included). */
  async addComment(communityId: string, postId: string, text: string): Promise<PostComment> {
    const comment = await port().community.comments.create(postId, { body: text });
    return mapCommentToPostComment(comment);
  },

  async toggleLikePost(communityId: string, postId: string): Promise<void> {
    const active = !LIKED_POST_IDS.has(postId);
    await port().community.reactions.set(postId, { reactionType: "like", active });
    if (active) {
      LIKED_POST_IDS.add(postId);
    } else {
      LIKED_POST_IDS.delete(postId);
    }
  },

  /** Deletes a post; the backend requires the author or a circle moderator. */
  async deletePost(communityId: string, postId: string): Promise<void> {
    await port().community.entries.delete(postId);
    const pending = PENDING_POSTS.get(communityId);
    if (pending) {
      PENDING_POSTS.set(communityId, pending.filter((post) => post.id !== postId));
    }
  },

  /** Resources are backend entries of kind "resource" within the circle. */
  async getResourcesByCommunity(communityId: string): Promise<Resource[]> {
    try {
      const entries = await port().community.feed.list({
        categoryId: communityId,
        kind: "resource",
      });
      return mergePendingResources(communityId, entries.map(mapEntryToResource));
    } catch {
      if (!isCommunityFeedsPortConfigured()) {
        return [];
      }
      const items = await listCircleFeedItems(
        getCommunityFeedsPort(),
        circleResourcesStreamKey(communityId),
      );
      return mergePendingResources(communityId, items.map(mapFeedItemToResource));
    }
  },

  async getGroupsByCommunity(communityId: string): Promise<CommunityGroup[]> {
    const groups = await port().community.groups.list(communityId);
    return groups.map(mapGroupToCommunityGroup);
  },

  async createGroup(
    communityId: string,
    group: Omit<CommunityGroup, "id" | "createdAt" | "communityId">,
  ): Promise<CommunityGroup> {
    const created = await port().community.groups.create(communityId, {
      name: group.name,
      platform: group.platform,
      description: group.description,
      memberCount: group.memberCount,
      qrCodes: group.qrCodes,
    });
    return mapGroupToCommunityGroup(created);
  },

  async updateGroup(
    communityId: string,
    groupId: string,
    data: Partial<CommunityGroup>,
  ): Promise<CommunityGroup> {
    const updated = await port().community.groups.update(communityId, groupId, {
      name: data.name,
      platform: data.platform,
      description: data.description,
      memberCount: data.memberCount,
      qrCodes: data.qrCodes,
    });
    return mapGroupToCommunityGroup(updated);
  },

  async updateCommunity(communityId: string, updates: Partial<Community>): Promise<void> {
    // Only defined fields are sent; the backend keeps the remaining fields.
    await port().community.categories.update(communityId, {
      ...(updates.name !== undefined ? { title: updates.name } : {}),
      ...(updates.description !== undefined ? { description: updates.description } : {}),
      ...(updates.avatar !== undefined ? { avatar: updates.avatar } : {}),
      ...(updates.coverImage !== undefined ? { coverImage: updates.coverImage } : {}),
      ...(updates.isPaid !== undefined ? { isPaid: updates.isPaid } : {}),
      ...(updates.memberLimit !== undefined ? { memberLimit: updates.memberLimit } : {}),
      ...(updates.price !== undefined ? { price: updates.price } : {}),
      ...(updates.revenueTarget !== undefined ? { revenueTarget: updates.revenueTarget } : {}),
      ...(updates.tags !== undefined ? { tags: updates.tags } : {}),
      ...(updates.tabs !== undefined ? { tabs: updates.tabs } : {}),
    });
  },

  async deleteGroup(communityId: string, groupId: string): Promise<void> {
    await port().community.groups.remove(communityId, groupId);
  },

  /** Lists purchasable (enabled) membership tiers of a circle. */
  async getMembershipTiers(communityId: string): Promise<MembershipTier[]> {
    const tiers = await port().community.tiers.list(communityId);
    return tiers.map(mapTierToMembershipTier);
  },

  /** Owner/admin: lists all tiers including unpublished ones. */
  async listAllMembershipTiers(communityId: string): Promise<MembershipTier[]> {
    const tiers = await port().community.tiers.listAll(communityId);
    return tiers.map(mapTierToMembershipTier);
  },

  /** Owner/admin: creates an unpublished membership tier. */
  async createMembershipTier(
    communityId: string,
    tier: Omit<MembershipTier, "id" | "categoryId" | "enabled">,
  ): Promise<MembershipTier> {
    const created = await port().community.tiers.create(communityId, {
      name: tier.name,
      description: tier.description,
      price: tier.price,
      durationDays: tier.durationDays,
      lifetimePrice: tier.lifetimePrice,
      benefits: tier.benefits,
      sortOrder: tier.sortOrder,
    });
    return mapTierToMembershipTier(created);
  },

  /** Owner/admin: updates a membership tier. */
  async updateMembershipTier(
    communityId: string,
    tierId: string,
    tier: Partial<Omit<MembershipTier, "id" | "categoryId" | "enabled">>,
  ): Promise<MembershipTier> {
    const updated = await port().community.tiers.update(communityId, tierId, {
      name: tier.name,
      description: tier.description,
      price: tier.price,
      durationDays: tier.durationDays,
      lifetimePrice: tier.lifetimePrice,
      benefits: tier.benefits,
      sortOrder: tier.sortOrder,
    });
    return mapTierToMembershipTier(updated);
  },

  /** Owner/admin: publishes a tier (registers its catalog package). */
  async publishMembershipTier(communityId: string, tierId: string): Promise<MembershipTier> {
    const tier = await port().community.tiers.publish(communityId, tierId);
    return mapTierToMembershipTier(tier);
  },

  /** Owner/admin: unpublishes a tier. */
  async unpublishMembershipTier(communityId: string, tierId: string): Promise<MembershipTier> {
    const tier = await port().community.tiers.unpublish(communityId, tierId);
    return mapTierToMembershipTier(tier);
  },

  /** Owner/admin: deletes a membership tier. */
  async deleteMembershipTier(communityId: string, tierId: string): Promise<void> {
    await port().community.tiers.remove(communityId, tierId);
  },

  /** Activates a paid membership after order payment (order verified server-side). */
  async activateMembership(
    communityId: string,
    orderId: string,
    tierId: string,
    packageId?: string,
  ): Promise<CommunityMember> {
    const member = await port().community.members.activate(communityId, {
      orderId,
      tierId,
      ...(packageId ? { packageId } : {}),
    });
    return mapMemberToCommunityMember(member);
  },
};
