export interface Community {
  id: string;
  name: string;
  description: string;
  coverImage: string;
  avatar?: string;
  /** Owner (creator) user id; absent for seeded/demo circles. */
  ownerId?: string;
  memberCount: number;
  /** Maximum member count; undefined means unlimited (默认不限制). */
  memberLimit?: number;
  postCount: number;
  /** Funding/raise target amount; undefined means no revenue cap. */
  revenueTarget?: number;
  /** Amount raised from paid memberships so far. */
  revenueRaised?: number;
  tags: string[];
  tabs?: string[];
  isJoined?: boolean;
  isPaid?: boolean;
  /** Agent-qualification circle: buying any tier grants the agent level. */
  isAgentCircle?: boolean;
  /** Recommended/pinned circle shown first in default ordering. */
  isRecommended?: boolean;
  price?: number;
  tiers?: MembershipTier[];
  /** Stored avatar reference (drive://) before the display URL is resolved. */
  rawAvatar?: string;
  /** Stored cover reference (drive://) before the display URL is resolved. */
  rawCoverImage?: string;
}

/** Circle membership tier (会员等级) shown on the purchase surface. */
export interface MembershipTier {
  id: string;
  categoryId: string;
  name: string;
  description?: string;
  /** Yearly price. */
  price: number;
  durationDays: number;
  /** Lifetime price; present when the tier supports a lifetime purchase. */
  lifetimePrice?: number;
  /** membership_package external id used as the order packageId. */
  catalogPackageId?: string;
  /** Lifetime package external id (order packageId for the lifetime purchase). */
  lifetimePackageId?: string;
  benefits: string[];
  /** Agent qualification level granted by this tier (absent = not an agent tier). */
  agentLevel?: string;
  enabled: boolean;
  sortOrder: number;
}

export interface PostComment {
  id: string;
  authorName: string;
  content: string;
  createdAt: string;
}

export interface Post {
  id: string;
  communityId: string;
  authorId: string;
  authorName: string;
  authorAvatar: string;
  content: string;
  images?: string[];
  createdAt: string;
  likes: number;
  comments: number;
  commentsList?: PostComment[];
  isLiked?: boolean;
  /** Backend review state; pending entries stay visible only to their author. */
  reviewState?: string;
  /** Entry tags (used by circle tab feeds such as news/docs/repos/software). */
  tags?: string[];
}

export interface QRCodeItem {
  url: string;
  description: string;
}

export interface CommunityGroup {
  id: string;
  communityId: string;
  name: string;
  platform: 'wechat' | 'qq' | 'feishu' | 'dingtalk' | 'telegram' | 'discord' | 'whatsapp' | 'other';
  description?: string;
  memberCount: number;
  qrCodeUrl?: string; // Legacy
  qrCodes?: QRCodeItem[];
  createdAt: string;
}

export interface Resource {
  id: string;
  communityId: string;
  title: string;
  type: string; // 'pdf', 'doc', 'link', 'video'
  size?: string;
  url: string;
  uploadedBy: string;
  createdAt: string;
  reviewState?: string;
}

export interface CommunityMember {
  id: string;
  communityId: string;
  name: string;
  avatar: string;
  role: 'owner' | 'admin' | 'member';
  joinDate: string;
  status: 'active' | 'muted' | 'banned';
  bio?: string;
  tierId?: string;
  tierName?: string;
  membershipExpiresAt?: string;
  agentLevel?: string;
}
