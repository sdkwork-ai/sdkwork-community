import { beforeEach, describe, expect, it } from "vitest";

import { createInMemoryCommunityAppSdkPort } from "@sdkwork/community-sdk-ports";
import {
  configureCommunityMediaRuntimePort,
  resetCommunityMediaRuntimePort,
} from "./communityMediaRuntimePort";
import type {
  SdkworkCommunityCategory,
  SdkworkCommunityComment,
  SdkworkCommunityEntry,
} from "@sdkwork/community-contracts";
import { CommunityService } from "./CommunityService";
import { configureCommunityRuntimePort, resetCommunityRuntimePort } from "./communityRuntimePort";

const AI_DEVELOPERS_ID = "1001";
const PRODUCT_MANAGERS_ID = "1002";

const CATEGORIES: readonly SdkworkCommunityCategory[] = [
  {
    coverImage: "https://example.test/cover.png",
    description: "AI 开发者联盟",
    enabled: true,
    id: AI_DEVELOPERS_ID,
    memberCount: 100,
    postCount: 1,
    priority: 0,
    slug: "ai",
    tabs: ["feeds", "resources", "groups"],
    tags: ["AI"],
    tenantId: "local",
    title: "AI 开发者联盟",
  },
  {
    coverImage: "https://example.test/cover-2.png",
    description: "产品经理交流圈",
    enabled: true,
    id: PRODUCT_MANAGERS_ID,
    isPaid: true,
    memberCount: 50,
    postCount: 0,
    priority: 0,
    price: 99,
    slug: "pm",
    tags: ["产品"],
    tenantId: "local",
    title: "产品经理交流圈",
  },
];

const COMMENTS: readonly SdkworkCommunityComment[] = [
  {
    author: { id: "user-2", name: "Alex" },
    body: "支持！",
    createdAt: "2026-05-28T02:10:00Z",
    entryId: "post_1",
    id: "cm_1",
    reviewState: "approved",
    tenantId: "local",
  },
];

const ENTRIES: readonly SdkworkCommunityEntry[] = [
  {
    author: { id: "user-1", name: "AI 极客" },
    body: "开源了一个 RAG 项目",
    categoryId: AI_DEVELOPERS_ID,
    id: "post_1",
    kind: "discussion",
    media: ["drive://spaces/s1/nodes/n1"],
    publishedAt: "2026-05-28T01:30:00Z",
    reviewState: "approved",
    stats: { commentCount: 1, reactionCount: 5 },
    tags: ["开源"],
    title: "RAG 项目",
  },
  {
    author: { id: "user-3", name: "资料君" },
    body: "AI 行业白皮书下载",
    categoryId: AI_DEVELOPERS_ID,
    id: "res_entry_1",
    kind: "resource",
    media: ["drive://spaces/s1/nodes/n2"],
    publishedAt: "2026-05-25T10:00:00Z",
    reviewState: "approved",
    stats: { commentCount: 0, reactionCount: 3 },
    tags: ["pdf"],
    title: "2026年AI行业发展白皮书.pdf",
  },
  {
    author: { id: "official-team", name: "官方团队" },
    body: JSON.stringify({
      status: "征集中",
      tag: "官方展映",
      participants: 613,
      background: "面向全球征集优质 AI 影像短片。",
      timeRange: "2026-06-30 00:00:00 - 2026-08-20 23:59:59",
      works: [{ id: "work-01", title: "《临水》" }],
    }),
    categoryId: AI_DEVELOPERS_ID,
    excerpt: "面向全球征集 AI 影像短片。",
    id: "activity_1",
    kind: "announcement",
    publishedAt: "2026-07-01T00:00:00Z",
    reviewState: "approved",
    stats: { commentCount: 0, reactionCount: 2 },
    tags: ["活动"],
    title: "AI 影像推荐单元",
  },
];

describe("CommunityService", () => {
  beforeEach(() => {
    resetCommunityRuntimePort();
    resetCommunityMediaRuntimePort();
    configureCommunityRuntimePort(
      createInMemoryCommunityAppSdkPort({
        categories: CATEGORIES,
        comments: COMMENTS,
        currentUserId: "user-1",
        entries: ENTRIES,
        memberships: [
          {
            communityId: AI_DEVELOPERS_ID,
            id: "user-1-membership",
            joinedAt: "2026-05-01T00:00:00Z",
            role: "owner",
            status: "active",
            user: { id: "user-1", name: "AI 极客" },
          },
        ],
      }),
    );
  });

  it("lists communities with joined state", async () => {
    const communities = await CommunityService.getCommunities();

    expect(communities).toHaveLength(2);
    expect(communities[0]).toMatchObject({
      id: AI_DEVELOPERS_ID,
      name: "AI 开发者联盟",
      memberCount: 100,
      isJoined: true,
      tabs: ["feeds", "resources", "groups"],
    });
    expect(communities[1]).toMatchObject({
      id: PRODUCT_MANAGERS_ID,
      isJoined: false,
      isPaid: true,
      price: 99,
    });
  });

  it("retrieves a community by id", async () => {
    const community = await CommunityService.getCommunityById(PRODUCT_MANAGERS_ID);
    expect(community?.name).toBe("产品经理交流圈");
    expect(community?.isJoined).toBe(false);
    await expect(CommunityService.getCommunityById("missing")).resolves.toBeUndefined();
  });

  it("creates a community and becomes its owner member", async () => {
    const created = await CommunityService.createCommunity({
      avatar: undefined,
      coverImage: "https://example.test/new.png",
      description: "新圈子",
      name: "独立开发者聚集地",
      tags: ["独立开发"],
    });

    expect(created.id).toBeTruthy();
    expect(created.id).toMatch(/^\d+$/);
    expect(created.isJoined).toBe(true);

    const members = await CommunityService.getMembersByCommunity(created.id);
    expect(members).toHaveLength(1);
    expect(members[0]).toMatchObject({ role: "owner", name: "Local User" });
  });

  it("joins and leaves a community", async () => {
    await CommunityService.joinCommunity(PRODUCT_MANAGERS_ID);

    const joined = await CommunityService.getCommunityById(PRODUCT_MANAGERS_ID);
    expect(joined?.isJoined).toBe(true);

    await CommunityService.leaveCommunity(PRODUCT_MANAGERS_ID);

    const left = await CommunityService.getCommunityById(PRODUCT_MANAGERS_ID);
    expect(left?.isJoined).toBe(false);
  });

  it("deletes a circle through the owner API", async () => {
    const created = await CommunityService.createCommunity({
      avatar: undefined,
      coverImage: "https://example.test/del.png",
      description: "待删除",
      name: "待删除圈子",
      tags: [],
    });

    await CommunityService.deleteCommunity(created.id);
    await expect(CommunityService.getCommunityById(created.id)).resolves.toBeUndefined();
  });

  it("lists posts with comments and maps backend media", async () => {
    const posts = await CommunityService.getPostsByCommunity(AI_DEVELOPERS_ID);

    expect(posts).toHaveLength(2);
    expect(posts.find((post) => post.id === "post_1")).toMatchObject({
      id: "post_1",
      communityId: AI_DEVELOPERS_ID,
      authorName: "AI 极客",
      content: "开源了一个 RAG 项目",
      likes: 5,
      comments: 1,
      isLiked: false,
      images: ["drive://spaces/s1/nodes/n1"],
    });
    const postOne = posts.find((post) => post.id === "post_1");
    expect(postOne?.commentsList).toEqual([
      expect.objectContaining({ authorName: "Alex", content: "支持！" }),
    ]);
  });

  it("renders JSON-structured entry bodies as readable text, never raw JSON", async () => {
    const posts = await CommunityService.getPostsByCommunity(AI_DEVELOPERS_ID);
    const activity = posts.find((post) => post.id === "activity_1");

    expect(activity).toBeDefined();
    expect(activity?.content).not.toContain("{");
    expect(activity?.content).not.toContain('"works"');
    expect(activity?.content).toContain("面向全球征集优质 AI 影像短片。");
    expect(activity?.content).toContain("2026-06-30 00:00:00 - 2026-08-20 23:59:59");
  });

  it("passes plain markdown bodies through unchanged", async () => {
    const posts = await CommunityService.getPostsByCommunity(AI_DEVELOPERS_ID);
    const plain = posts.find((post) => post.id === "post_1");
    expect(plain?.content).toBe("开源了一个 RAG 项目");
  });

  it("creates posts and comments, and toggles likes", async () => {
    const post = await CommunityService.createPost(AI_DEVELOPERS_ID, "第一行\n正文内容");
    expect(post.communityId).toBe(AI_DEVELOPERS_ID);
    expect(post.id).toMatch(/^\d+$/);

    const createdComment = await CommunityService.addComment(AI_DEVELOPERS_ID, post.id, "评论内容");
    // The backend mints the comment id — never a client-side temp id.
    expect(createdComment.id).toMatch(/^\d+$/);
    expect(createdComment.content).toBe("评论内容");

    await CommunityService.toggleLikePost(AI_DEVELOPERS_ID, post.id);
    const posts = await CommunityService.getPostsByCommunity(AI_DEVELOPERS_ID);
    const created = posts.find((item) => item.id === post.id);
    expect(created?.isLiked).toBe(true);
  });

  it("uploads post images through the host media runtime and stores drive urls", async () => {
    configureCommunityMediaRuntimePort({
      uploadImages: async (files) =>
        files.map((file, index) => `drive://spaces/up/nodes/${index}-${file.name}`),
    });

    const file = new File(["image-bytes"], "photo.png", { type: "image/png" });
    const post = await CommunityService.createPost(AI_DEVELOPERS_ID, "带图动态", [file]);

    expect(post.images).toEqual(["drive://spaces/up/nodes/0-photo.png"]);
    const posts = await CommunityService.getPostsByCommunity(AI_DEVELOPERS_ID);
    const persisted = posts.find((item) => item.id === post.id);
    expect(persisted?.images).toEqual(["drive://spaces/up/nodes/0-photo.png"]);
  });

  it("refuses media posts when the host has no media runtime", async () => {
    const file = new File(["image-bytes"], "photo.png", { type: "image/png" });
    await expect(
      CommunityService.createPost(AI_DEVELOPERS_ID, "带图动态", [file]),
    ).rejects.toThrow(/media runtime port is not configured/);
  });

  it("manages members: role, status and removal", async () => {
    await CommunityService.updateMemberRole(AI_DEVELOPERS_ID, "user-1-membership", "admin");
    await CommunityService.updateMemberStatus(AI_DEVELOPERS_ID, "user-1-membership", "muted");

    let members = await CommunityService.getMembersByCommunity(AI_DEVELOPERS_ID);
    expect(members[0]).toMatchObject({ role: "admin", status: "muted" });

    await CommunityService.removeMember(AI_DEVELOPERS_ID, "user-1-membership");
    members = await CommunityService.getMembersByCommunity(AI_DEVELOPERS_ID);
    expect(members).toHaveLength(0);
  });

  it("manages groups: create, update and delete", async () => {
    const group = await CommunityService.createGroup(AI_DEVELOPERS_ID, {
      memberCount: 120,
      name: "AI 开发者交流群",
      platform: "wechat",
      qrCodes: [{ url: "https://example.test/qr.png", description: "扫码加入" }],
    });
    expect(group).toMatchObject({ name: "AI 开发者交流群", platform: "wechat", qrCodeUrl: "https://example.test/qr.png" });
    expect(group.id).toMatch(/^\d+$/);

    const updated = await CommunityService.updateGroup(AI_DEVELOPERS_ID, group.id, { name: "新群名" });
    expect(updated.name).toBe("新群名");

    await CommunityService.deleteGroup(AI_DEVELOPERS_ID, group.id);
    await expect(CommunityService.getGroupsByCommunity(AI_DEVELOPERS_ID)).resolves.toEqual([]);
  });

  it("updates a community", async () => {
    await CommunityService.updateCommunity(AI_DEVELOPERS_ID, { name: "AI 开发者联盟 2.0" });

    const community = await CommunityService.getCommunityById(AI_DEVELOPERS_ID);
    expect(community?.name).toBe("AI 开发者联盟 2.0");
  });

  it("updates only the given community fields", async () => {
    await CommunityService.updateCommunity(AI_DEVELOPERS_ID, { tabs: ["feeds", "groups"] });

    const community = await CommunityService.getCommunityById(AI_DEVELOPERS_ID);
    expect(community?.name).toBe("AI 开发者联盟");
    expect(community?.tabs).toEqual(["feeds", "groups"]);
  });

  it("lists resources from backend entries of kind resource", async () => {
    const resources = await CommunityService.getResourcesByCommunity(AI_DEVELOPERS_ID);
    expect(resources).toEqual([
      {
        id: "res_entry_1",
        communityId: AI_DEVELOPERS_ID,
        title: "2026年AI行业发展白皮书.pdf",
        type: "pdf",
        url: "drive://spaces/s1/nodes/n2",
        uploadedBy: "资料君",
        createdAt: "2026-05-25T10:00:00Z",
        reviewState: "approved",
      },
    ]);
    await expect(CommunityService.getResourcesByCommunity(PRODUCT_MANAGERS_ID)).resolves.toEqual([]);
  });
});
