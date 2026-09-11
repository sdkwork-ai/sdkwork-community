import type { CommunityGroupQr } from './community-group-qr';
export interface CommunityGroupResponse {
    id: string;
    tenantId: string;
    categoryId: string;
    name: string;
    platform: string;
    description?: string;
    memberCount: string;
    qrCodes: CommunityGroupQr[];
    createdAt: string;
    updatedAt: string;
}
//# sourceMappingURL=community-group-response.d.ts.map