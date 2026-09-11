export interface CommunityCategory {
    id: string;
    tenantId: string;
    slug: string;
    title: string;
    description?: string;
    coverImage?: string;
    avatar?: string;
    ownerId?: string;
    memberCount: string;
    memberLimit?: string;
    postCount: string;
    isPaid: boolean;
    price?: number;
    revenueRaised?: number;
    revenueTarget?: number;
    tags: string[];
    tabs?: string[];
    priority: number;
    enabled: boolean;
    isJoined: boolean;
}
//# sourceMappingURL=community-category.d.ts.map