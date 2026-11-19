#!/usr/bin/env node
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import {
  sdkWorkEnvelopeComponentSchemas,
  successResponseSchemaRef,
} from "../../sdkwork-specs/tools/lib/openapi-envelope-schemas.mjs";
import { alignOpenApiOperationPatterns } from "../../sdkwork-specs/tools/lib/align-api-operation-patterns.mjs";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, "..");
const outputDir = path.join(workspaceRoot, "generated", "openapi");
const apiAuthorityTargets = [
  ["community-app-api.openapi.json", "apis/app-api/community/openapi.json"],
  ["community-backend-api.openapi.json", "apis/backend-api/community/openapi.json"],
  ["community-open-api.openapi.json", "apis/open-api/community/openapi.json"],
];
const OWNER = "sdkwork-community";
const DOMAIN = "community";
const HTTP_METHODS = new Set(["delete", "get", "patch", "post", "put"]);
const iamModuleManifest = JSON.parse(readFileSync(
  path.join(workspaceRoot, "specs", "iam.module.manifest.json"),
  "utf8",
));
const permissionCatalog = new Set(
  (iamModuleManifest.permissions?.catalog ?? []).map((entry) => entry.code),
);

const domainSchemas = {
  CommunityCategory: {
    type: "object",
    additionalProperties: false,
    required: ["id", "tenantId", "slug", "title", "memberCount", "postCount", "isPaid", "isJoined", "tags", "priority", "enabled"],
    properties: {
      id: { type: "string" },
      tenantId: { type: "string" },
      slug: { type: "string" },
      title: { type: "string" },
      description: { type: "string" },
      coverImage: { type: "string" },
      avatar: { type: "string" },
      ownerId: { type: "string" },
      memberCount: { type: "integer", format: "int64" },
      memberLimit: { type: "integer", format: "int64" },
      postCount: { type: "integer", format: "int64" },
      isPaid: { type: "boolean" },
      price: { type: "number", format: "double" },
      revenueRaised: { type: "number", format: "double" },
      revenueTarget: { type: "number", format: "double" },
      tags: { type: "array", items: { type: "string" } },
      tabs: { type: "array", items: { type: "string" } },
      priority: { type: "integer" },
      enabled: { type: "boolean" },
      isJoined: { type: "boolean" },
    },
  },
  CommunityCircleCommand: {
    type: "object",
    additionalProperties: false,
    required: ["title"],
    properties: {
      title: { type: "string" },
      description: { type: "string" },
      coverImage: { type: "string" },
      avatar: { type: "string" },
      isPaid: { type: "boolean" },
      memberLimit: { type: "integer", format: "int64" },
      price: { type: "number", format: "double" },
      revenueTarget: { type: "number", format: "double" },
      tags: { type: "array", items: { type: "string" } },
      tabs: { type: "array", items: { type: "string" } },
    },
  },
  CommunityMemberResponse: {
    type: "object",
    additionalProperties: false,
    required: ["id", "tenantId", "categoryId", "userId", "userName", "role", "status", "joinedAt"],
    properties: {
      id: { type: "string" },
      tenantId: { type: "string" },
      categoryId: { type: "string" },
      userId: { type: "string" },
      userName: { type: "string" },
      role: { type: "string" },
      status: { type: "string" },
      bio: { type: "string" },
      tierId: { type: "string" },
      tierName: { type: "string" },
      membershipExpiresAt: { type: "string", format: "date-time" },
      agentLevel: { type: "string" },
      lastOrderId: { type: "string" },
      joinedAt: { type: "string", format: "date-time" },
    },
  },
  CommunityMemberPatchCommand: {
    type: "object",
    additionalProperties: false,
    properties: {
      role: { type: "string" },
      status: { type: "string" },
    },
  },
  CommunityGroupQr: {
    type: "object",
    additionalProperties: false,
    required: ["url"],
    properties: {
      url: { type: "string" },
      description: { type: "string" },
    },
  },
  CommunityGroupResponse: {
    type: "object",
    additionalProperties: false,
    required: ["id", "tenantId", "categoryId", "name", "platform", "memberCount", "qrCodes", "createdAt", "updatedAt"],
    properties: {
      id: { type: "string" },
      tenantId: { type: "string" },
      categoryId: { type: "string" },
      name: { type: "string" },
      platform: { type: "string" },
      description: { type: "string" },
      memberCount: { type: "integer", format: "int64" },
      qrCodes: { type: "array", items: { $ref: "#/components/schemas/CommunityGroupQr" } },
      createdAt: { type: "string" },
      updatedAt: { type: "string" },
    },
  },
  CommunityGroupCommand: {
    type: "object",
    additionalProperties: false,
    required: ["name", "platform"],
    properties: {
      name: { type: "string" },
      platform: { type: "string" },
      description: { type: "string" },
      memberCount: { type: "integer", format: "int64" },
      qrCodes: { type: "array", items: { $ref: "#/components/schemas/CommunityGroupQr" } },
    },
  },
  CommunityMembershipTier: {
    type: "object",
    additionalProperties: false,
    required: ["id", "tenantId", "categoryId", "name", "price", "durationDays", "benefits", "sortOrder", "enabled"],
    properties: {
      id: { type: "string" },
      tenantId: { type: "string" },
      categoryId: { type: "string" },
      name: { type: "string" },
      description: { type: "string" },
      price: { type: "number", format: "double" },
      durationDays: { type: "integer", format: "int64" },
      lifetimePrice: { type: "number", format: "double" },
      lifetimePackageId: { type: "string" },
      benefits: { type: "array", items: { type: "string" } },
      agentLevel: { type: "string" },
      catalogPackageId: { type: "string" },
      sortOrder: { type: "integer", format: "int64" },
      enabled: { type: "boolean" },
    },
  },
  CommunityTierCommand: {
    type: "object",
    additionalProperties: false,
    required: ["name", "price"],
    properties: {
      name: { type: "string" },
      description: { type: "string" },
      price: { type: "number", format: "double" },
      durationDays: { type: "integer", format: "int64" },
      lifetimePrice: { type: "number", format: "double" },
      benefits: { type: "array", items: { type: "string" } },
      agentLevel: { type: "string" },
      sortOrder: { type: "integer", format: "int64" },
    },
  },
  CommunityActivateMembershipCommand: {
    type: "object",
    additionalProperties: false,
    required: ["orderId", "tierId"],
    properties: {
      orderId: { type: "string" },
      tierId: { type: "string" },
    },
  },
  CommunityAuthor: {
    type: "object",
    additionalProperties: false,
    required: ["id", "name"],
    properties: {
      id: { type: "string" },
      name: { type: "string" },
      avatarUrl: { type: "string", format: "uri" },
    },
  },
  CommunityEntryKind: {
    type: "string",
    enum: ["announcement", "discussion", "question", "resource", "service"],
  },
  CommunityReviewState: {
    type: "string",
    enum: ["approved", "draft", "flagged", "pending-review", "rejected"],
  },
  CommunityStats: {
    type: "object",
    additionalProperties: false,
    properties: {
      commentCount: { type: "integer" },
      reactionCount: { type: "integer" },
      shareCount: { type: "integer" },
      viewCount: { type: "integer" },
    },
  },
  CommunityEntry: {
    type: "object",
    additionalProperties: false,
    required: [
      "id",
      "tenantId",
      "categoryId",
      "author",
      "slug",
      "kind",
      "title",
      "reviewState",
      "stats",
    ],
    properties: {
      id: { type: "string" },
      tenantId: { type: "string" },
      categoryId: { type: "string" },
      categoryLabel: { type: "string" },
      author: { $ref: "#/components/schemas/CommunityAuthor" },
      slug: { type: "string" },
      kind: { $ref: "#/components/schemas/CommunityEntryKind" },
      title: { type: "string" },
      excerpt: { type: "string" },
      body: { type: "string" },
      reviewState: { $ref: "#/components/schemas/CommunityReviewState" },
      isFeatured: { type: "boolean" },
      isPinned: { type: "boolean" },
      hasAcceptedAnswer: { type: "boolean" },
      stats: { $ref: "#/components/schemas/CommunityStats" },
      tags: { type: "array", items: { type: "string" } },
      media: { type: "array", items: { type: "string" } },
      publishedAt: { type: "string", format: "date-time" },
      lastActivityAt: { type: "string", format: "date-time" },
      updatedAt: { type: "string", format: "date-time" },
    },
  },
  CommunityComment: {
    type: "object",
    additionalProperties: false,
    required: ["id", "tenantId", "entryId", "author", "body", "reviewState", "createdAt"],
    properties: {
      id: { type: "string" },
      tenantId: { type: "string" },
      entryId: { type: "string" },
      author: { $ref: "#/components/schemas/CommunityAuthor" },
      body: { type: "string" },
      reviewState: { $ref: "#/components/schemas/CommunityReviewState" },
      isAcceptedAnswer: { type: "boolean" },
      createdAt: { type: "string", format: "date-time" },
      updatedAt: { type: "string", format: "date-time" },
    },
  },
  CommunityEntryCommand: {
    type: "object",
    additionalProperties: false,
    required: ["categoryId", "kind", "title"],
    properties: {
      categoryId: { type: "string" },
      kind: { $ref: "#/components/schemas/CommunityEntryKind" },
      title: { type: "string" },
      excerpt: { type: "string" },
      body: { type: "string" },
      tags: { type: "array", items: { type: "string" } },
      media: { type: "array", items: { type: "string" } },
    },
  },
  CommunityCategoryCommand: {
    type: "object",
    additionalProperties: false,
    required: ["slug", "title"],
    properties: {
      slug: { type: "string" },
      title: { type: "string" },
      description: { type: "string" },
      priority: { type: "integer" },
      enabled: { type: "boolean" },
    },
  },
  CommunityCommentCommand: {
    type: "object",
    additionalProperties: false,
    required: ["body"],
    properties: {
      body: { type: "string" },
    },
  },
  CommunityReactionCommand: {
    type: "object",
    additionalProperties: false,
    required: ["reactionType", "active"],
    properties: {
      reactionType: { type: "string" },
      active: { type: "boolean" },
    },
  },
  CommunityReactionSetAccepted: {
    type: "object",
    additionalProperties: false,
    required: ["accepted", "reactionCount"],
    properties: {
      accepted: { type: "boolean" },
      resourceId: { type: "string" },
      status: { type: "string" },
      reactionCount: { type: "integer" },
    },
  },
  CommunityModerationCommand: {
    type: "object",
    additionalProperties: false,
    required: ["reviewState"],
    properties: {
      reviewState: { $ref: "#/components/schemas/CommunityReviewState" },
      reason: { type: "string" },
    },
  },
  CommunityFeatureCommand: {
    type: "object",
    additionalProperties: false,
    properties: {
      featured: { type: "boolean" },
    },
  },
  CommunityPinCommand: {
    type: "object",
    additionalProperties: false,
    properties: {
      pinned: { type: "boolean" },
    },
  },
  CommunityPublicationReadiness: {
    type: "object",
    additionalProperties: false,
    required: ["ready", "degraded", "issues"],
    properties: {
      ready: { type: "boolean" },
      degraded: { type: "boolean" },
      issues: { type: "array", items: { type: "string" } },
    },
  },
};

const schemas = {
  ...sdkWorkEnvelopeComponentSchemas,
  ...domainSchemas,
};

const appRoutes = [
  route("get", "/app/v3/api/community/categories", "categories.list", false),
  route("post", "/app/v3/api/community/categories", "categories.create", false, [], "CommunityCircleCommand"),
  route("get", "/app/v3/api/community/categories/{categoryId}", "categories.retrieve", false, [pathParam("categoryId")]),
  route("patch", "/app/v3/api/community/categories/{categoryId}", "categories.update", false, [pathParam("categoryId")], "CommunityCircleCommand"),
  route("delete", "/app/v3/api/community/categories/{categoryId}", "categories.delete", false, [pathParam("categoryId")]),
  route("post", "/app/v3/api/community/categories/{categoryId}/join", "categories.join", false, [pathParam("categoryId")]),
  route("get", "/app/v3/api/community/categories/{categoryId}/members", "members.list", false, [pathParam("categoryId")]),
  route("get", "/app/v3/api/community/categories/{categoryId}/members/current", "members.current", false, [pathParam("categoryId")]),
  route("patch", "/app/v3/api/community/categories/{categoryId}/members/{memberId}", "members.update", false, [pathParam("categoryId"), pathParam("memberId")], "CommunityMemberPatchCommand"),
  route("delete", "/app/v3/api/community/categories/{categoryId}/members/{memberId}", "members.remove", false, [pathParam("categoryId"), pathParam("memberId")]),
  route("post", "/app/v3/api/community/categories/{categoryId}/members/activate", "members.activate", false, [pathParam("categoryId")], "CommunityActivateMembershipCommand"),
  route("get", "/app/v3/api/community/categories/{categoryId}/tiers", "tiers.list", false, [pathParam("categoryId"), queryParam("include_disabled", { type: "boolean" })]),
  route("post", "/app/v3/api/community/categories/{categoryId}/tiers", "tiers.create", false, [pathParam("categoryId")], "CommunityTierCommand"),
  route("patch", "/app/v3/api/community/categories/{categoryId}/tiers/{tierId}", "tiers.update", false, [pathParam("categoryId"), pathParam("tierId")], "CommunityTierCommand"),
  route("delete", "/app/v3/api/community/categories/{categoryId}/tiers/{tierId}", "tiers.delete", false, [pathParam("categoryId"), pathParam("tierId")]),
  route("post", "/app/v3/api/community/categories/{categoryId}/tiers/{tierId}/publish", "tiers.publish", false, [pathParam("categoryId"), pathParam("tierId")]),
  route("post", "/app/v3/api/community/categories/{categoryId}/tiers/{tierId}/unpublish", "tiers.unpublish", false, [pathParam("categoryId"), pathParam("tierId")]),
  route("get", "/app/v3/api/community/categories/{categoryId}/groups", "groups.list", false, [pathParam("categoryId")]),
  route("post", "/app/v3/api/community/categories/{categoryId}/groups", "groups.create", false, [pathParam("categoryId")], "CommunityGroupCommand"),
  route("patch", "/app/v3/api/community/categories/{categoryId}/groups/{groupId}", "groups.update", false, [pathParam("categoryId"), pathParam("groupId")], "CommunityGroupCommand"),
  route("delete", "/app/v3/api/community/categories/{categoryId}/groups/{groupId}", "groups.remove", false, [pathParam("categoryId"), pathParam("groupId")]),
  route("get", "/app/v3/api/community/feed", "feed.list", true, listParams()),
  route("get", "/app/v3/api/community/entries/{entryId}", "entries.retrieve", false, [pathParam("entryId")]),
  route("get", "/app/v3/api/community/entries/{entryId}/recommendations", "entries.recommendations.list", false, [pathParam("entryId")]),
  route("post", "/app/v3/api/community/entries", "entries.create", false, [], "CommunityEntryCommand"),
  route("patch", "/app/v3/api/community/entries/{entryId}", "entries.update", false, [pathParam("entryId")], "CommunityEntryCommand"),
  route("delete", "/app/v3/api/community/entries/{entryId}", "entries.delete", false, [pathParam("entryId")]),
  route("get", "/app/v3/api/community/entries/{entryId}/publication_readiness", "entries.publicationReadiness.retrieve", false, [pathParam("entryId")]),
  route("post", "/app/v3/api/community/entries/{entryId}/reactions", "reactions.set", false, [pathParam("entryId")], "CommunityReactionCommand"),
  route("get", "/app/v3/api/community/entries/{entryId}/comments", "comments.list", false, [pathParam("entryId")]),
  route("post", "/app/v3/api/community/entries/{entryId}/comments", "comments.create", false, [pathParam("entryId")], "CommunityCommentCommand"),
];

const backendRoutes = [
  route("get", "/backend/v3/api/community/categories", "categories.management.list", false),
  route("post", "/backend/v3/api/community/categories", "categories.create", false, [], "CommunityCategoryCommand"),
  route("patch", "/backend/v3/api/community/categories/{categoryId}", "categories.update", false, [pathParam("categoryId")], "CommunityCategoryCommand"),
  route("delete", "/backend/v3/api/community/categories/{categoryId}", "categories.delete", false, [pathParam("categoryId")]),
  route("patch", "/backend/v3/api/community/circles/{categoryId}", "circles.update", false, [pathParam("categoryId")], "CommunityCircleCommand"),
  route("get", "/backend/v3/api/community/entries", "entries.management.list", false, listParams()),
  route("post", "/backend/v3/api/community/entries/{entryId}/moderation", "entries.moderation.create", false, [pathParam("entryId")], "CommunityModerationCommand"),
  route("post", "/backend/v3/api/community/entries/{entryId}/feature", "entries.feature", false, [pathParam("entryId")], "CommunityFeatureCommand"),
  route("post", "/backend/v3/api/community/entries/{entryId}/pin", "entries.pin", false, [pathParam("entryId")], "CommunityPinCommand"),
  route("delete", "/backend/v3/api/community/entries/{entryId}", "entries.delete", false, [pathParam("entryId")]),
  route("get", "/backend/v3/api/community/moderation/queue", "moderation.queue.list", false),
  route("post", "/backend/v3/api/community/recommendations/rebuild", "recommendations.rebuild", false),
  route("get", "/backend/v3/api/community/members", "members.management.list", false, [categoryQueryParam()]),
  route("patch", "/backend/v3/api/community/members/{memberId}", "members.update", false, [pathParam("memberId"), categoryQueryParam()], "CommunityMemberPatchCommand"),
  route("delete", "/backend/v3/api/community/members/{memberId}", "members.delete", false, [pathParam("memberId"), categoryQueryParam()]),
  route("get", "/backend/v3/api/community/groups", "groups.management.list", false, [categoryQueryParam()]),
  route("post", "/backend/v3/api/community/groups", "groups.create", false, [categoryQueryParam()], "CommunityGroupCommand"),
  route("patch", "/backend/v3/api/community/groups/{groupId}", "groups.update", false, [pathParam("groupId"), categoryQueryParam()], "CommunityGroupCommand"),
  route("delete", "/backend/v3/api/community/groups/{groupId}", "groups.delete", false, [pathParam("groupId"), categoryQueryParam()]),
  route("get", "/backend/v3/api/community/tiers", "tiers.management.list", false, [categoryQueryParam(), queryParam("enabled_only", { type: "boolean" })]),
  route("post", "/backend/v3/api/community/tiers", "tiers.create", false, [categoryQueryParam()], "CommunityTierCommand"),
  route("patch", "/backend/v3/api/community/tiers/{tierId}", "tiers.update", false, [pathParam("tierId"), categoryQueryParam()], "CommunityTierCommand"),
  route("delete", "/backend/v3/api/community/tiers/{tierId}", "tiers.delete", false, [pathParam("tierId"), categoryQueryParam()]),
  route("post", "/backend/v3/api/community/tiers/{tierId}/publish", "tiers.publish", false, [pathParam("tierId"), categoryQueryParam()]),
  route("post", "/backend/v3/api/community/tiers/{tierId}/unpublish", "tiers.unpublish", false, [pathParam("tierId"), categoryQueryParam()]),
];

// feature/pin bodies are optional for backward compatibility (absent body keeps
// the historical "set to true" behavior), so the generated SDK must not treat
// them as required.
for (const operationId of ["entries.feature", "entries.pin"]) {
  const operation = backendRoutes.find((item) => item.operation.operationId === operationId);
  operation.operation.requestBody.required = false;
}

const openRoutes = [
  route("get", "/community/v3/api/categories", "categories.public.list", true),
  route("get", "/community/v3/api/feed", "feed.public.list", true, listParams()),
  route("get", "/community/v3/api/entries/{entryId}", "entries.public.retrieve", true, [pathParam("entryId")]),
  route("get", "/community/v3/api/entries/by_slug/{slug}", "entries.publicBySlug.retrieve", true, [pathParam("slug")]),
];

// The community feed surfaces are superseded by the standard feeds stream
// system (sdkwork-feeds `streams.items.list` with community/moments/
// inspiration stream keys). Keep them as deprecated compatibility layers
// until consumers finish migrating; do not extend them.
for (const operationId of ["feed.list", "feed.public.list"]) {
  for (const routes of [appRoutes, openRoutes]) {
    const operation = routes.find((item) => item.operation.operationId === operationId);
    if (operation) {
      operation.operation.deprecated = true;
      operation.operation["x-sdkwork-deprecated"] = true;
      operation.operation.summary =
        `${operation.operation.summary} (deprecated: use the standard feeds stream system)`;
    }
  }
}

function ref(name) {
  return { $ref: `#/components/schemas/${name}` };
}

function envelopeSchemaRef(method, operationId) {
  if (method === "delete") {
    return "#/components/schemas/SdkWorkCommandResponse";
  }
  return successResponseSchemaRef({ method, operationId });
}

function pathParam(name) {
  return {
    name,
    in: "path",
    required: true,
    schema: { type: "string", minLength: 1 },
  };
}

function queryParam(name, schema = { type: "string" }) {
  return {
    name,
    in: "query",
    required: false,
    schema,
  };
}

function categoryQueryParam() {
  return {
    name: "category_id",
    in: "query",
    required: true,
    schema: { type: "string", minLength: 1 },
  };
}

function listParams() {
  return [
    queryParam("category_id"),
    queryParam("kind"),
    queryParam("q"),
    queryParam("review_state"),
    queryParam("tag"),
    queryParam("page", { type: "integer", minimum: 1, default: 1 }),
    queryParam("page_size", { type: "integer", minimum: 1, maximum: 200, default: 20 }),
  ];
}

function route(method, pathKey, operationId, isPublic, parameters = [], bodySchemaName = null) {
  const apiSurface = pathKey.startsWith("/community/v3/api")
    ? "open-api"
    : pathKey.startsWith("/backend/v3/api")
      ? "backend-api"
      : "app-api";
  return {
    method,
    path: pathKey,
    operation: {
      tags: ["community"],
      summary: `Community ${operationId}`,
      operationId,
      parameters,
      ...(bodySchemaName ? {
        requestBody: {
          required: true,
          content: {
            "application/json": {
              schema: ref(bodySchemaName),
            },
          },
        },
      } : {}),
      responses: {
        200: {
          description: "OK",
          content: {
            "application/json": {
              schema: { $ref: envelopeSchemaRef(method, operationId) },
            },
          },
        },
        400: problemResponse(),
        401: problemResponse(),
      },
      security: isPublic ? [] : [{ AuthToken: [], AccessToken: [] }],
      "x-sdkwork-owner": OWNER,
      "x-sdkwork-api-authority": "",
      "x-sdkwork-domain": DOMAIN,
      "x-sdkwork-resource": operationId.split(".")[0],
      "x-sdkwork-public": isPublic,
      "x-sdkwork-auth-mode": isPublic ? "anonymous" : "dual-token",
      "x-sdkwork-request-context": "WebRequestContext",
      "x-sdkwork-api-surface": apiSurface,
      "x-sdkwork-standard-profile": "sdkwork-v3",
    },
  };
}

function problemResponse() {
  return {
    description: "Problem detail",
    content: {
      "application/problem+json": {
        schema: ref("ProblemDetail"),
      },
    },
  };
}

function documentFor({ authority, routes, serverUrl, title }) {
  const paths = {};
  for (const item of routes) {
    paths[item.path] ??= {};
    item.operation["x-sdkwork-api-authority"] = authority;
    paths[item.path][item.method] = item.operation;
  }
  // `/members/current` returns 200 with `data.item: null` when the current
  // user has not joined (absence, not an error), so clients can distinguish
  // "not a member" from a failed lookup. Contract must mark the item
  // nullable accordingly.
  const currentMemberOperation =
    paths["/app/v3/api/community/categories/{categoryId}/members/current"]?.get;
  if (currentMemberOperation) {
    currentMemberOperation.responses[200] = {
      description: "OK",
      content: {
        "application/json": {
          schema: {
            allOf: [
              { $ref: "#/components/schemas/SdkWorkApiResponse" },
              {
                type: "object",
                required: ["data"],
                properties: {
                  data: {
                    type: "object",
                    required: ["item"],
                    properties: {
                      item: {
                        oneOf: [
                          { $ref: "#/components/schemas/CommunityMemberResponse" },
                          { type: "null" },
                        ],
                      },
                    },
                  },
                },
              },
            ],
          },
        },
      },
    };
  }
  const document = {
    openapi: "3.1.2",
    info: {
      title,
      version: "1.0.0",
      "x-sdkwork-owner": OWNER,
      "x-sdkwork-api-authority": authority,
    },
    servers: [{ url: serverUrl }],
    tags: [{ name: "community", description: "Community API resources.", "x-sdk-nested-resource-surface": true }],
    paths,
    components: {
      ...(routes.some((item) => !item.operation["x-sdkwork-public"])
        ? {
          securitySchemes: {
            AuthToken: {
              type: "http",
              scheme: "bearer",
              bearerFormat: "JWT",
            },
            AccessToken: {
              type: "apiKey",
              in: "header",
              name: "Access-Token",
            },
          },
        }
        : {
          securitySchemes: {
            ApiKey: {
              type: "apiKey",
              in: "header",
              name: "X-API-Key",
            },
          },
        }),
      schemas,
    },
    "x-sdkwork-owner": OWNER,
    "x-sdkwork-api-authority": authority,
    "x-sdkwork-domain": DOMAIN,
    "x-sdkwork-standard-profile": "sdkwork-v3",
  };
  return applyPermissionContract(
    alignOpenApiOperationPatterns(document).document,
    authority,
  );
}

function applyPermissionContract(document, authority) {
  const authorityContract = (iamModuleManifest.permissions?.openapiAuthorities ?? [])
    .find((entry) => entry.apiAuthority === authority);
  const permissionsByOperation = new Map();
  for (const mapping of authorityContract?.operationPermissions ?? []) {
    if (permissionsByOperation.has(mapping.operationId)) {
      throw new Error(`${authority} duplicates permission mapping for ${mapping.operationId}`);
    }
    if (!permissionCatalog.has(mapping.permission)) {
      throw new Error(`${authority} maps ${mapping.operationId} to unknown permission ${mapping.permission}`);
    }
    permissionsByOperation.set(mapping.operationId, mapping.permission);
  }

  const seenOperations = new Set();
  for (const pathItem of Object.values(document.paths ?? {})) {
    for (const [method, operation] of Object.entries(pathItem ?? {})) {
      if (!HTTP_METHODS.has(method)) {
        continue;
      }
      const permission = permissionsByOperation.get(operation.operationId);
      if (operation["x-sdkwork-public"] === true) {
        if (permission) {
          throw new Error(`${authority} public operation ${operation.operationId} must not require ${permission}`);
        }
        continue;
      }
      if (permission) {
        operation["x-sdkwork-permission"] = permission;
      }
      seenOperations.add(operation.operationId);
    }
  }

  for (const operationId of permissionsByOperation.keys()) {
    if (!seenOperations.has(operationId)) {
      throw new Error(`${authority} permission mapping references missing operation ${operationId}`);
    }
  }
  return document;
}

function parseArgs(argv) {
  return {
    check: argv.includes("--check"),
  };
}

const args = parseArgs(process.argv.slice(2));
const docs = [
  ["community-open-api.openapi.json", documentFor({ authority: "sdkwork-community-open-api", routes: openRoutes, serverUrl: "http://127.0.0.1:18082", title: "SDKWork Community Open API" })],
  ["community-app-api.openapi.json", documentFor({ authority: "sdkwork-community-app-api", routes: appRoutes, serverUrl: "http://127.0.0.1:18080", title: "SDKWork Community App API" })],
  ["community-backend-api.openapi.json", documentFor({ authority: "sdkwork-community-backend-api", routes: backendRoutes, serverUrl: "http://127.0.0.1:18080", title: "SDKWork Community Backend API" })],
];

if (!args.check) {
  mkdirSync(outputDir, { recursive: true });
  for (const [fileName, document] of docs) {
    const payload = `${JSON.stringify(document, null, 2)}\n`;
    writeFileSync(path.join(outputDir, fileName), payload, "utf8");
    const authorityTarget = apiAuthorityTargets.find(([source]) => source === fileName);
    if (authorityTarget) {
      const authorityPath = path.join(workspaceRoot, authorityTarget[1]);
      mkdirSync(path.dirname(authorityPath), { recursive: true });
      writeFileSync(authorityPath, payload, "utf8");
    }
  }
}

process.stdout.write(`[community_openapi_export] ok app=${appRoutes.length} backend=${backendRoutes.length} open=${openRoutes.length}\n`);

