import { backendApiPath } from './paths';
import type { ApiRequestOptions, HttpClient } from '../http/client';

import type { CommunityCategoryCommand, CommunityCircleCommand, CommunityFeatureCommand, CommunityGroupCommand, CommunityMemberPatchCommand, CommunityModerationCommand, CommunityPinCommand, CommunityTierCommand, SdkWorkPageData } from '../types';


export interface CommunityTiersManagementListParams {
  categoryId: string;
  enabledOnly?: boolean;
}

export class CommunityTiersManagementApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Community tiers.management.list */
  async list(params: CommunityTiersManagementListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'category_id', value: params.categoryId, style: 'form', explode: true, allowReserved: false },
      { name: 'enabled_only', value: params.enabledOnly, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageData>(appendQueryString(backendApiPath(`/community/tiers`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }
}

export interface CommunityTiersCreateParams {
  categoryId: string;
}

export interface CommunityTiersUpdateParams {
  categoryId: string;
}

export interface CommunityTiersDeleteParams {
  categoryId: string;
}

export interface CommunityTiersPublishParams {
  categoryId: string;
}

export interface CommunityTiersUnpublishParams {
  categoryId: string;
}

export class CommunityTiersApi {
  private client: HttpClient;
  public readonly management: CommunityTiersManagementApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.management = new CommunityTiersManagementApi(client);
  }


/** Community tiers.create */
  async create(body: CommunityTierCommand, params: CommunityTiersCreateParams, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>> {
    const query = buildQueryString([
      { name: 'category_id', value: params.categoryId, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<Record<string, unknown>>(appendQueryString(backendApiPath(`/community/tiers`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Community tiers.update */
  async update(tierId: string, body: CommunityTierCommand, params: CommunityTiersUpdateParams, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>> {
    const query = buildQueryString([
      { name: 'category_id', value: params.categoryId, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<Record<string, unknown>>(appendQueryString(backendApiPath(`/community/tiers/${serializePathParameter(tierId, { name: 'tierId', style: 'simple', explode: false })}`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'PATCH' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Community tiers.delete */
  async delete(tierId: string, params: CommunityTiersDeleteParams, requestOptions?: ApiRequestOptions): Promise<void> {
    const query = buildQueryString([
      { name: 'category_id', value: params.categoryId, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<void>(appendQueryString(backendApiPath(`/community/tiers/${serializePathParameter(tierId, { name: 'tierId', style: 'simple', explode: false })}`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'DELETE' as any });
  }

/** Community tiers.publish */
  async publish(tierId: string, params: CommunityTiersPublishParams, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>> {
    const query = buildQueryString([
      { name: 'category_id', value: params.categoryId, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<Record<string, unknown>>(appendQueryString(backendApiPath(`/community/tiers/${serializePathParameter(tierId, { name: 'tierId', style: 'simple', explode: false })}/publish`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, sdkworkUnwrapKind: 'item' });
  }

/** Community tiers.unpublish */
  async unpublish(tierId: string, params: CommunityTiersUnpublishParams, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>> {
    const query = buildQueryString([
      { name: 'category_id', value: params.categoryId, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<Record<string, unknown>>(appendQueryString(backendApiPath(`/community/tiers/${serializePathParameter(tierId, { name: 'tierId', style: 'simple', explode: false })}/unpublish`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, sdkworkUnwrapKind: 'item' });
  }
}

export interface CommunityGroupsManagementListParams {
  categoryId: string;
}

export class CommunityGroupsManagementApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Community groups.management.list */
  async list(params: CommunityGroupsManagementListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'category_id', value: params.categoryId, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageData>(appendQueryString(backendApiPath(`/community/groups`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }
}

export interface CommunityGroupsCreateParams {
  categoryId: string;
}

export interface CommunityGroupsUpdateParams {
  categoryId: string;
}

export interface CommunityGroupsDeleteParams {
  categoryId: string;
}

export class CommunityGroupsApi {
  private client: HttpClient;
  public readonly management: CommunityGroupsManagementApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.management = new CommunityGroupsManagementApi(client);
  }


/** Community groups.create */
  async create(body: CommunityGroupCommand, params: CommunityGroupsCreateParams, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>> {
    const query = buildQueryString([
      { name: 'category_id', value: params.categoryId, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<Record<string, unknown>>(appendQueryString(backendApiPath(`/community/groups`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Community groups.update */
  async update(groupId: string, body: CommunityGroupCommand, params: CommunityGroupsUpdateParams, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>> {
    const query = buildQueryString([
      { name: 'category_id', value: params.categoryId, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<Record<string, unknown>>(appendQueryString(backendApiPath(`/community/groups/${serializePathParameter(groupId, { name: 'groupId', style: 'simple', explode: false })}`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'PATCH' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Community groups.delete */
  async delete(groupId: string, params: CommunityGroupsDeleteParams, requestOptions?: ApiRequestOptions): Promise<void> {
    const query = buildQueryString([
      { name: 'category_id', value: params.categoryId, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<void>(appendQueryString(backendApiPath(`/community/groups/${serializePathParameter(groupId, { name: 'groupId', style: 'simple', explode: false })}`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'DELETE' as any });
  }
}

export interface CommunityMembersManagementListParams {
  categoryId: string;
}

export class CommunityMembersManagementApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Community members.management.list */
  async list(params: CommunityMembersManagementListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'category_id', value: params.categoryId, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageData>(appendQueryString(backendApiPath(`/community/members`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }
}

export interface CommunityMembersUpdateParams {
  categoryId: string;
}

export interface CommunityMembersDeleteParams {
  categoryId: string;
}

export class CommunityMembersApi {
  private client: HttpClient;
  public readonly management: CommunityMembersManagementApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.management = new CommunityMembersManagementApi(client);
  }


/** Community members.update */
  async update(memberId: string, body: CommunityMemberPatchCommand, params: CommunityMembersUpdateParams, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>> {
    const query = buildQueryString([
      { name: 'category_id', value: params.categoryId, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<Record<string, unknown>>(appendQueryString(backendApiPath(`/community/members/${serializePathParameter(memberId, { name: 'memberId', style: 'simple', explode: false })}`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'PATCH' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Community members.delete */
  async delete(memberId: string, params: CommunityMembersDeleteParams, requestOptions?: ApiRequestOptions): Promise<void> {
    const query = buildQueryString([
      { name: 'category_id', value: params.categoryId, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<void>(appendQueryString(backendApiPath(`/community/members/${serializePathParameter(memberId, { name: 'memberId', style: 'simple', explode: false })}`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'DELETE' as any });
  }
}

export class CommunityRecommendationsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Community recommendations.rebuild */
  async rebuild(requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>> {
    return this.client.request<Record<string, unknown>>(backendApiPath(`/community/recommendations/rebuild`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class CommunityModerationQueueApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Community moderation.queue.list */
  async list(requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    return this.client.request<SdkWorkPageData>(backendApiPath(`/community/moderation/queue`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }
}

export class CommunityModerationApi {
  public readonly queue: CommunityModerationQueueApi;

  constructor(client: HttpClient) {
    this.queue = new CommunityModerationQueueApi(client);
  }

}

export class CommunityEntriesModerationApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Community entries.moderation.create */
  async create(entryId: string, body: CommunityModerationCommand, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>> {
    return this.client.request<Record<string, unknown>>(backendApiPath(`/community/entries/${serializePathParameter(entryId, { name: 'entryId', style: 'simple', explode: false })}/moderation`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export interface CommunityEntriesManagementListParams {
  categoryId?: string;
  kind?: string;
  q?: string;
  reviewState?: string;
  tag?: string;
  page?: number;
  pageSize?: number;
}

export class CommunityEntriesManagementApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Community entries.management.list */
  async list(params?: CommunityEntriesManagementListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    const query = buildQueryString([
      { name: 'category_id', value: params?.categoryId, style: 'form', explode: true, allowReserved: false },
      { name: 'kind', value: params?.kind, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
      { name: 'review_state', value: params?.reviewState, style: 'form', explode: true, allowReserved: false },
      { name: 'tag', value: params?.tag, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageData>(appendQueryString(backendApiPath(`/community/entries`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }
}

export class CommunityEntriesApi {
  private client: HttpClient;
  public readonly management: CommunityEntriesManagementApi;
  public readonly moderation: CommunityEntriesModerationApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.management = new CommunityEntriesManagementApi(client);
    this.moderation = new CommunityEntriesModerationApi(client);
  }


/** Community entries.feature */
  async feature(entryId: string, body?: CommunityFeatureCommand, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>> {
    return this.client.request<Record<string, unknown>>(backendApiPath(`/community/entries/${serializePathParameter(entryId, { name: 'entryId', style: 'simple', explode: false })}/feature`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, ...(body !== undefined ? { body, contentType: 'application/json' } : {}), sdkworkUnwrapKind: 'item' });
  }

/** Community entries.pin */
  async pin(entryId: string, body?: CommunityPinCommand, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>> {
    return this.client.request<Record<string, unknown>>(backendApiPath(`/community/entries/${serializePathParameter(entryId, { name: 'entryId', style: 'simple', explode: false })}/pin`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, ...(body !== undefined ? { body, contentType: 'application/json' } : {}), sdkworkUnwrapKind: 'item' });
  }

/** Community entries.delete */
  async delete(entryId: string, requestOptions?: ApiRequestOptions): Promise<void> {
    return this.client.request<void>(backendApiPath(`/community/entries/${serializePathParameter(entryId, { name: 'entryId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'DELETE' as any });
  }
}

export class CommunityCirclesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Community circles.update */
  async update(categoryId: string, body: CommunityCircleCommand, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>> {
    return this.client.request<Record<string, unknown>>(backendApiPath(`/community/circles/${serializePathParameter(categoryId, { name: 'categoryId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'PATCH' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class CommunityCategoriesManagementApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Community categories.management.list */
  async list(requestOptions?: ApiRequestOptions): Promise<SdkWorkPageData> {
    return this.client.request<SdkWorkPageData>(backendApiPath(`/community/categories`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }
}

export class CommunityCategoriesApi {
  private client: HttpClient;
  public readonly management: CommunityCategoriesManagementApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.management = new CommunityCategoriesManagementApi(client);
  }


/** Community categories.create */
  async create(body: CommunityCategoryCommand, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>> {
    return this.client.request<Record<string, unknown>>(backendApiPath(`/community/categories`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Community categories.update */
  async update(categoryId: string, body: CommunityCategoryCommand, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>> {
    return this.client.request<Record<string, unknown>>(backendApiPath(`/community/categories/${serializePathParameter(categoryId, { name: 'categoryId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'PATCH' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Community categories.delete */
  async delete(categoryId: string, requestOptions?: ApiRequestOptions): Promise<void> {
    return this.client.request<void>(backendApiPath(`/community/categories/${serializePathParameter(categoryId, { name: 'categoryId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'DELETE' as any });
  }
}

export class CommunityApi {
  public readonly categories: CommunityCategoriesApi;
  public readonly circles: CommunityCirclesApi;
  public readonly entries: CommunityEntriesApi;
  public readonly moderation: CommunityModerationApi;
  public readonly recommendations: CommunityRecommendationsApi;
  public readonly members: CommunityMembersApi;
  public readonly groups: CommunityGroupsApi;
  public readonly tiers: CommunityTiersApi;

  constructor(client: HttpClient) {
    this.categories = new CommunityCategoriesApi(client);
    this.circles = new CommunityCirclesApi(client);
    this.entries = new CommunityEntriesApi(client);
    this.moderation = new CommunityModerationApi(client);
    this.recommendations = new CommunityRecommendationsApi(client);
    this.members = new CommunityMembersApi(client);
    this.groups = new CommunityGroupsApi(client);
    this.tiers = new CommunityTiersApi(client);
  }

}

export function createCommunityApi(client: HttpClient): CommunityApi {
  return new CommunityApi(client);
}

function appendQueryString(path: string, rawQueryString: string): string {
  const query = rawQueryString.replace(/^\?+/, '');
  if (!query) {
    return path;
  }
  return path.includes('?') ? `${path}&${query}` : `${path}?${query}`;
}

interface PathParameterSpec {
  name: string;
  style: string;
  explode: boolean;
}

function serializePathParameter(value: unknown, spec: PathParameterSpec): string {
  if (value === undefined || value === null) {
    return '';
  }

  const style = spec.style || 'simple';
  if (Array.isArray(value)) {
    return serializePathArray(spec.name, value, style, spec.explode);
  }
  if (typeof value === 'object') {
    return serializePathObject(spec.name, value as Record<string, unknown>, style, spec.explode);
  }
  return pathPrefix(spec.name, style, false) + encodePathValue(serializePathPrimitive(value));
}

function serializePathArray(name: string, values: unknown[], style: string, explode: boolean): string {
  const serialized = values
    .filter((item) => item !== undefined && item !== null)
    .map((item) => encodePathValue(serializePathPrimitive(item)));
  if (serialized.length === 0) {
    return pathPrefix(name, style, false);
  }
  if (style === 'matrix') {
    return explode
      ? serialized.map((item) => `;${name}=${item}`).join('')
      : `;${name}=${serialized.join(',')}`;
  }
  return pathPrefix(name, style, false) + serialized.join(explode ? '.' : ',');
}

function serializePathObject(name: string, value: Record<string, unknown>, style: string, explode: boolean): string {
  const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
  if (entries.length === 0) {
    return pathPrefix(name, style, true);
  }
  if (style === 'matrix') {
    return explode
      ? entries.map(([key, entryValue]) => `;${encodePathValue(key)}=${encodePathValue(serializePathPrimitive(entryValue))}`).join('')
      : `;${name}=${entries.flatMap(([key, entryValue]) => [encodePathValue(key), encodePathValue(serializePathPrimitive(entryValue))]).join(',')}`;
  }
  const serialized = explode
    ? entries.map(([key, entryValue]) => `${encodePathValue(key)}=${encodePathValue(serializePathPrimitive(entryValue))}`).join(style === 'label' ? '.' : ',')
    : entries.flatMap(([key, entryValue]) => [encodePathValue(key), encodePathValue(serializePathPrimitive(entryValue))]).join(',');
  return pathPrefix(name, style, true) + serialized;
}

function pathPrefix(name: string, style: string, _objectValue: boolean): string {
  if (style === 'label') return '.';
  if (style === 'matrix') return `;${name}`;
  return '';
}

function encodePathValue(value: string): string {
  return encodeURIComponent(value);
}

function serializePathPrimitive(value: unknown): string {
  if (value instanceof Date) {
    return value.toISOString();
  }
  if (typeof value === 'object') {
    return JSON.stringify(value);
  }
  return String(value);
}
interface QueryParameterSpec {
  name: string;
  value: unknown;
  style: string;
  explode: boolean;
  allowReserved: boolean;
  contentType?: string;
}

function buildQueryString(parameters: QueryParameterSpec[]): string {
  const pairs: string[] = [];
  for (const parameter of parameters) {
    appendSerializedParameter(pairs, parameter);
  }
  return pairs.join('&');
}

function appendSerializedParameter(pairs: string[], parameter: QueryParameterSpec): void {
  if (parameter.value === undefined || parameter.value === null) {
    return;
  }

  if (parameter.contentType) {
    pairs.push(`${encodeQueryComponent(parameter.name)}=${encodeQueryValue(JSON.stringify(parameter.value), parameter.allowReserved)}`);
    return;
  }

  const style = parameter.style || 'form';
  if (style === 'deepObject') {
    appendDeepObjectParameter(pairs, parameter.name, parameter.value, parameter.allowReserved);
    return;
  }

  if (Array.isArray(parameter.value)) {
    appendArrayParameter(pairs, parameter.name, parameter.value, style, parameter.explode, parameter.allowReserved);
    return;
  }

  if (typeof parameter.value === 'object') {
    appendObjectParameter(pairs, parameter.name, parameter.value as Record<string, unknown>, style, parameter.explode, parameter.allowReserved);
    return;
  }

  pairs.push(`${encodeQueryComponent(parameter.name)}=${encodeQueryValue(serializePrimitive(parameter.value), parameter.allowReserved)}`);
}

function appendArrayParameter(
  pairs: string[],
  name: string,
  value: unknown[],
  style: string,
  explode: boolean,
  allowReserved: boolean,
): void {
  const values = value
    .filter((item) => item !== undefined && item !== null)
    .map((item) => serializePrimitive(item));
  if (values.length === 0) {
    return;
  }

  if (style === 'form' && explode) {
    for (const item of values) {
      pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(item, allowReserved)}`);
    }
    return;
  }

  pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(values.join(','), allowReserved)}`);
}

function appendObjectParameter(
  pairs: string[],
  name: string,
  value: Record<string, unknown>,
  style: string,
  explode: boolean,
  allowReserved: boolean,
): void {
  const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
  if (entries.length === 0) {
    return;
  }

  if (style === 'form' && explode) {
    for (const [key, entryValue] of entries) {
      pairs.push(`${encodeQueryComponent(key)}=${encodeQueryValue(serializePrimitive(entryValue), allowReserved)}`);
    }
    return;
  }

  const serialized = entries.flatMap(([key, entryValue]) => [key, serializePrimitive(entryValue)]).join(',');
  pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(serialized, allowReserved)}`);
}

function appendDeepObjectParameter(
  pairs: string[],
  name: string,
  value: unknown,
  allowReserved: boolean,
): void {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(serializePrimitive(value), allowReserved)}`);
    return;
  }

  for (const [key, entryValue] of Object.entries(value as Record<string, unknown>)) {
    if (entryValue === undefined || entryValue === null) {
      continue;
    }
    pairs.push(`${encodeQueryComponent(`${name}[${key}]`)}=${encodeQueryValue(serializePrimitive(entryValue), allowReserved)}`);
  }
}

function serializePrimitive(value: unknown): string {
  if (value instanceof Date) {
    return value.toISOString();
  }
  if (typeof value === 'object') {
    return JSON.stringify(value);
  }
  return String(value);
}

function encodeQueryComponent(value: string): string {
  return encodeURIComponent(value);
}

function encodeQueryValue(value: string, allowReserved: boolean): string {
  const encoded = encodeURIComponent(value);
  if (!allowReserved) {
    return encoded;
  }
  return encoded.replace(/%3A/gi, ':')
    .replace(/%2F/gi, '/')
    .replace(/%3F/gi, '?')
    .replace(/%23/gi, '#')
    .replace(/%5B/gi, '[')
    .replace(/%5D/gi, ']')
    .replace(/%40/gi, '@')
    .replace(/%21/gi, '!')
    .replace(/%24/gi, '$')
    .replace(/%26/gi, '&')
    .replace(/%27/gi, "'")
    .replace(/%28/gi, '(')
    .replace(/%29/gi, ')')
    .replace(/%2A/gi, '*')
    .replace(/%2B/gi, '+')
    .replace(/%2C/gi, ',')
    .replace(/%3B/gi, ';')
    .replace(/%3D/gi, '=');
}
