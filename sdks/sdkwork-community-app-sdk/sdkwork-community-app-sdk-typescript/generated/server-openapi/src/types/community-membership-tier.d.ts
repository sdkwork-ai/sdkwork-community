export interface CommunityMembershipTier {
    id: string;
    tenantId: string;
    categoryId: string;
    name: string;
    description?: string;
    price: number;
    durationDays: string;
    lifetimePrice?: number;
    lifetimePackageId?: string;
    benefits: string[];
    agentLevel?: string;
    catalogPackageId?: string;
    sortOrder: string;
    enabled: boolean;
}
//# sourceMappingURL=community-membership-tier.d.ts.map