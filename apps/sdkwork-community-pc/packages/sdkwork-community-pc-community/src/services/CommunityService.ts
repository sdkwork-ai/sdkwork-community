import { isBlank, trim, truncate } from "@sdkwork/utils";
import type { SdkworkCommunityAppSdkPort } from "@sdkwork/community-sdk-ports";
import type { FeedItem, SdkworkFeedsClient } from "@sdkwork/feeds-sdk";
import type { SdkworkCommunityCategory, SdkworkCommunityEntry } from "@sdkwork/community-contracts";
import { getCommunityPcHost } from "../host/adapter";

export const PC_COMMUNITY_SUPPORTED_TABS = ["feeds"] as const;
export type PcCommunitySupportedTab = (typeof PC_COMMUNITY_SUPPORTED_TABS)[number];

export const PC_COMMUNITY_REACTION_TYPE = "like";

export interface Community {
  id: string;
  name: string;
  description: string;
  avatar: string;
  cover: string;
  membersCount: number;
  tags: string[];
  tabs?: string[];
}

export interface Post {
  id: string;
  communityId: string;
  author: {
    id: string;
    name: string;
    avatar: string;
  };
  content: string;
  likes: number;
  comments: number;
  createdAt: string;
}

export interface CommunityComment {
  author: {
    avatar: string;
    id: string;
    name: string;
  };
  body: string;
  createdAt: string;
  entryId: string;
  id: string;
}

export interface PostReactionResult {
  active: boolean;
  reactionCount: number;
}

/** Shipped App API surface consumed by PC community UI. */
export interface CommunityService {
  createComment(communityId: string, postId: string, content: string): Promise<void>;
  createPost(communityId: string, content: string): Promise<Post>;
  deletePost(postId: string): Promise<void>;
  getComments(postId: string): Promise<CommunityComment[]>;
  getCommunities(): Promise<Community[]>;
  getCommunity(id: string): Promise<Community | undefined>;
  getPosts(communityId: string): Promise<Post[]>;
  setPostReaction(postId: string, active: boolean): Promise<PostReactionResult>;
}

export const PC_COMMUNITY_CONTENT_REQUIRED = "pc community content is required";
export const PC_COMMUNITY_MEDIA_UNAVAILABLE = "pc community media contract is not available";

interface CommunityServiceOptions {
  port?: SdkworkCommunityAppSdkPort;
  /** Optional standard feeds stream client; falls back to community feed.list. */
  feedsClient?: SdkworkFeedsClient;
}

function failClosed(message: string): never {
  throw new Error(message);
}

function requireNonBlankContent(value: string): string {
  const normalized = trim(value);
  if (isBlank(normalized)) {
    failClosed(PC_COMMUNITY_CONTENT_REQUIRED);
  }
  return normalized;
}

function mapCategoryToCommunity(category: SdkworkCommunityCategory): Community {
  return {
    id: category.id,
    name: category.title,
    description: category.description ?? "",
    avatar: "",
    cover: "",
    membersCount: 0,
    tags: [],
    tabs: [...PC_COMMUNITY_SUPPORTED_TABS],
  };
}

function mapEntryToCommunity(entry: SdkworkCommunityEntry): Community {
  return {
    id: entry.id,
    name: entry.title,
    description: entry.excerpt ?? "",
    avatar: entry.author.avatar?.publicUrl ?? "",
    cover: "",
    membersCount: entry.stats.viewCount ?? 0,
    tags: entry.tags ? [...entry.tags] : [],
    tabs: [...PC_COMMUNITY_SUPPORTED_TABS],
  };
}

function mapEntryToPost(entry: SdkworkCommunityEntry): Post {
  return {
    id: entry.id,
    communityId: entry.categoryId,
    author: {
      id: entry.author.id,
      name: entry.author.name,
      avatar: entry.author.avatar?.publicUrl ?? "",
    },
    content: entry.excerpt ?? entry.title,
    likes: entry.stats.reactionCount ?? 0,
    comments: entry.stats.commentCount ?? 0,
    createdAt: String(entry.publishedAt ?? entry.lastActivityAt ?? new Date().toISOString()),
  };
}

function mapFeedItemToPost(item: FeedItem): Post {
  const streamCommunityId = (item.streamKey ?? "").replace(/^community-/, "").replace(/-resources$/, "");
  return {
    id: item.id,
    communityId: streamCommunityId,
    author: {
      id: item.author?.id ?? "",
      name: item.author?.name ?? "",
      avatar: item.author?.avatarUrl ?? "",
    },
    content: (item.excerpt ?? item.title ?? "").trim(),
    likes: item.reactionCount ?? 0,
    comments: item.commentCount ?? 0,
    createdAt: String(item.publishedAt ?? item.createdAt ?? ""),
  };
}

class SdkworkCommunityPcService implements CommunityService {
  constructor(options: CommunityServiceOptions = {}) {
    // Host access MUST stay lazy: this module is evaluated during the static
    // import chain of the embedding app (e.g. sdkwork-im-pc main.tsx), while
    // `configureCommunityPcHost()` only runs later in that app's bootstrap.
    // Resolving the host here threw
    // "Community PC host adapter is not configured" at module-evaluation time.
    this.portFactory = () => options.port ?? getCommunityPcHost().createAppSdkPort();
    this.explicitFeedsClientFactory = options.feedsClient
      ? () => options.feedsClient as SdkworkFeedsClient
      : null;
  }

  private readonly portFactory: () => SdkworkCommunityAppSdkPort;
  private readonly explicitFeedsClientFactory: (() => SdkworkFeedsClient) | null;

  private port(): SdkworkCommunityAppSdkPort {
    return this.portFactory();
  }

  private feedsClient(): SdkworkFeedsClient | null {
    if (this.explicitFeedsClientFactory) {
      return this.explicitFeedsClientFactory();
    }
    // Resolved lazily at first data fetch — after the embedding app has
    // configured the host adapter.
    const hostFeeds = getCommunityPcHost().createFeedsSdkClient;
    return hostFeeds ? hostFeeds() : null;
  }

  async getCommunities(): Promise<Community[]> {
    const categories = await this.port().community.categories.list();
    return categories.filter((category) => category.enabled !== false).map(mapCategoryToCommunity);
  }

  async getCommunity(id: string): Promise<Community | undefined> {
    try {
      const entry = await this.port().community.entries.retrieve(id);
      if (entry.id) {
        return mapEntryToCommunity(entry);
      }
    } catch {
      // Fall back to category lookup for category-scoped community ids.
    }
    const categories = await this.getCommunities();
    return categories.find((community) => community.id === id);
  }

  async getPosts(communityId: string): Promise<Post[]> {
    const feeds = this.feedsClient();
    if (feeds) {
      // Standard feeds stream: community-{circleId} carries all circle posts.
      const page = await feeds.feeds.streams.items.list(`community-${communityId}`, {
        pageSize: 100,
      });
      return (page.items as unknown as FeedItem[]).map(mapFeedItemToPost);
    }
    // Migration fallback: legacy community feed surface.
    const entries = await this.port().community.feed.list({ categoryId: communityId });
    return entries.map(mapEntryToPost);
  }

  async getComments(postId: string): Promise<CommunityComment[]> {
    const comments = await this.port().community.comments.list(postId);
    return comments.map((comment) => ({
      id: comment.id,
      entryId: comment.entryId,
      body: comment.body,
      createdAt: String(comment.createdAt),
      author: {
        id: comment.author.id,
        name: comment.author.name,
        avatar: comment.author.avatar?.publicUrl ?? "",
      },
    }));
  }

  async createPost(communityId: string, content: string): Promise<Post> {
    const normalized = requireNonBlankContent(content);
    const entry = await this.port().community.entries.create({
      categoryId: communityId,
      kind: "discussion",
      title: truncate(normalized, 120) || "Untitled",
      body: normalized,
      excerpt: truncate(normalized, 240),
    });
    return mapEntryToPost(entry);
  }

  async createComment(
    _communityId: string,
    postId: string,
    content: string,
  ): Promise<void> {
    const normalized = requireNonBlankContent(content);
    await this.port().community.comments.create(postId, { body: normalized });
  }

  async setPostReaction(postId: string, active: boolean): Promise<PostReactionResult> {
    const result = await this.port().community.reactions.set(postId, {
      reactionType: PC_COMMUNITY_REACTION_TYPE,
      active,
    });
    return {
      active: result.status === "active",
      reactionCount: result.reactionCount,
    };
  }

  async deletePost(postId: string): Promise<void> {
    await this.port().community.entries.delete(postId);
  }
}

export function createCommunityService(options: CommunityServiceOptions = {}): CommunityService {
  return new SdkworkCommunityPcService(options);
}

export const communityService = createCommunityService();
