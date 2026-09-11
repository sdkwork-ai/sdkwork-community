import { backendApiPath } from './paths';
export class CommunityTiersManagementApi {
    client;
    constructor(client) {
        this.client = client;
    }
    /** Community tiers.management.list */
    async list(params, requestOptions) {
        const query = buildQueryString([
            { name: 'categoryId', value: params.categoryId, style: 'form', explode: true, allowReserved: false },
            { name: 'enabledOnly', value: params.enabledOnly, style: 'form', explode: true, allowReserved: false },
        ]);
        return this.client.request(appendQueryString(backendApiPath(`/community/tiers`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET', sdkworkUnwrapKind: 'page' });
    }
}
export class CommunityTiersApi {
    client;
    management;
    constructor(client) {
        this.client = client;
        this.management = new CommunityTiersManagementApi(client);
    }
    /** Community tiers.create */
    async create(body, params, requestOptions) {
        const query = buildQueryString([
            { name: 'categoryId', value: params.categoryId, style: 'form', explode: true, allowReserved: false },
        ]);
        return this.client.request(appendQueryString(backendApiPath(`/community/tiers`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST', body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
    }
    /** Community tiers.update */
    async update(tierId, body, params, requestOptions) {
        const query = buildQueryString([
            { name: 'categoryId', value: params.categoryId, style: 'form', explode: true, allowReserved: false },
        ]);
        return this.client.request(appendQueryString(backendApiPath(`/community/tiers/${serializePathParameter(tierId, { name: 'tierId', style: 'simple', explode: false })}`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'PATCH', body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
    }
    /** Community tiers.delete */
    async delete(tierId, params, requestOptions) {
        const query = buildQueryString([
            { name: 'categoryId', value: params.categoryId, style: 'form', explode: true, allowReserved: false },
        ]);
        return this.client.request(appendQueryString(backendApiPath(`/community/tiers/${serializePathParameter(tierId, { name: 'tierId', style: 'simple', explode: false })}`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'DELETE' });
    }
    /** Community tiers.publish */
    async publish(tierId, params, requestOptions) {
        const query = buildQueryString([
            { name: 'categoryId', value: params.categoryId, style: 'form', explode: true, allowReserved: false },
        ]);
        return this.client.request(appendQueryString(backendApiPath(`/community/tiers/${serializePathParameter(tierId, { name: 'tierId', style: 'simple', explode: false })}/publish`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST', sdkworkUnwrapKind: 'item' });
    }
    /** Community tiers.unpublish */
    async unpublish(tierId, params, requestOptions) {
        const query = buildQueryString([
            { name: 'categoryId', value: params.categoryId, style: 'form', explode: true, allowReserved: false },
        ]);
        return this.client.request(appendQueryString(backendApiPath(`/community/tiers/${serializePathParameter(tierId, { name: 'tierId', style: 'simple', explode: false })}/unpublish`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST', sdkworkUnwrapKind: 'item' });
    }
}
export class CommunityGroupsManagementApi {
    client;
    constructor(client) {
        this.client = client;
    }
    /** Community groups.management.list */
    async list(params, requestOptions) {
        const query = buildQueryString([
            { name: 'categoryId', value: params.categoryId, style: 'form', explode: true, allowReserved: false },
        ]);
        return this.client.request(appendQueryString(backendApiPath(`/community/groups`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET', sdkworkUnwrapKind: 'page' });
    }
}
export class CommunityGroupsApi {
    client;
    management;
    constructor(client) {
        this.client = client;
        this.management = new CommunityGroupsManagementApi(client);
    }
    /** Community groups.create */
    async create(body, params, requestOptions) {
        const query = buildQueryString([
            { name: 'categoryId', value: params.categoryId, style: 'form', explode: true, allowReserved: false },
        ]);
        return this.client.request(appendQueryString(backendApiPath(`/community/groups`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST', body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
    }
    /** Community groups.update */
    async update(groupId, body, params, requestOptions) {
        const query = buildQueryString([
            { name: 'categoryId', value: params.categoryId, style: 'form', explode: true, allowReserved: false },
        ]);
        return this.client.request(appendQueryString(backendApiPath(`/community/groups/${serializePathParameter(groupId, { name: 'groupId', style: 'simple', explode: false })}`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'PATCH', body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
    }
    /** Community groups.delete */
    async delete(groupId, params, requestOptions) {
        const query = buildQueryString([
            { name: 'categoryId', value: params.categoryId, style: 'form', explode: true, allowReserved: false },
        ]);
        return this.client.request(appendQueryString(backendApiPath(`/community/groups/${serializePathParameter(groupId, { name: 'groupId', style: 'simple', explode: false })}`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'DELETE' });
    }
}
export class CommunityMembersManagementApi {
    client;
    constructor(client) {
        this.client = client;
    }
    /** Community members.management.list */
    async list(params, requestOptions) {
        const query = buildQueryString([
            { name: 'categoryId', value: params.categoryId, style: 'form', explode: true, allowReserved: false },
        ]);
        return this.client.request(appendQueryString(backendApiPath(`/community/members`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET', sdkworkUnwrapKind: 'page' });
    }
}
export class CommunityMembersApi {
    client;
    management;
    constructor(client) {
        this.client = client;
        this.management = new CommunityMembersManagementApi(client);
    }
    /** Community members.update */
    async update(memberId, body, params, requestOptions) {
        const query = buildQueryString([
            { name: 'categoryId', value: params.categoryId, style: 'form', explode: true, allowReserved: false },
        ]);
        return this.client.request(appendQueryString(backendApiPath(`/community/members/${serializePathParameter(memberId, { name: 'memberId', style: 'simple', explode: false })}`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'PATCH', body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
    }
    /** Community members.delete */
    async delete(memberId, params, requestOptions) {
        const query = buildQueryString([
            { name: 'categoryId', value: params.categoryId, style: 'form', explode: true, allowReserved: false },
        ]);
        return this.client.request(appendQueryString(backendApiPath(`/community/members/${serializePathParameter(memberId, { name: 'memberId', style: 'simple', explode: false })}`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'DELETE' });
    }
}
export class CommunityRecommendationsApi {
    client;
    constructor(client) {
        this.client = client;
    }
    /** Community recommendations.rebuild */
    async rebuild(requestOptions) {
        return this.client.request(backendApiPath(`/community/recommendations/rebuild`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST', sdkworkUnwrapKind: 'item' });
    }
}
export class CommunityModerationQueueApi {
    client;
    constructor(client) {
        this.client = client;
    }
    /** Community moderation.queue.list */
    async list(requestOptions) {
        return this.client.request(backendApiPath(`/community/moderation/queue`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET', sdkworkUnwrapKind: 'page' });
    }
}
export class CommunityModerationApi {
    queue;
    constructor(client) {
        this.queue = new CommunityModerationQueueApi(client);
    }
}
export class CommunityEntriesModerationApi {
    client;
    constructor(client) {
        this.client = client;
    }
    /** Community entries.moderation.create */
    async create(entryId, body, requestOptions) {
        return this.client.request(backendApiPath(`/community/entries/${serializePathParameter(entryId, { name: 'entryId', style: 'simple', explode: false })}/moderation`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST', body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
    }
}
export class CommunityEntriesManagementApi {
    client;
    constructor(client) {
        this.client = client;
    }
    /** Community entries.management.list */
    async list(params, requestOptions) {
        const query = buildQueryString([
            { name: 'categoryId', value: params?.categoryId, style: 'form', explode: true, allowReserved: false },
            { name: 'kind', value: params?.kind, style: 'form', explode: true, allowReserved: false },
            { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
            { name: 'reviewState', value: params?.reviewState, style: 'form', explode: true, allowReserved: false },
            { name: 'tag', value: params?.tag, style: 'form', explode: true, allowReserved: false },
            { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
            { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
        ]);
        return this.client.request(appendQueryString(backendApiPath(`/community/entries`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET', sdkworkUnwrapKind: 'page' });
    }
}
export class CommunityEntriesApi {
    client;
    management;
    moderation;
    constructor(client) {
        this.client = client;
        this.management = new CommunityEntriesManagementApi(client);
        this.moderation = new CommunityEntriesModerationApi(client);
    }
    /** Community entries.feature */
    async feature(entryId, body, requestOptions) {
        return this.client.request(backendApiPath(`/community/entries/${serializePathParameter(entryId, { name: 'entryId', style: 'simple', explode: false })}/feature`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST', ...(body !== undefined ? { body, contentType: 'application/json' } : {}), sdkworkUnwrapKind: 'item' });
    }
    /** Community entries.pin */
    async pin(entryId, body, requestOptions) {
        return this.client.request(backendApiPath(`/community/entries/${serializePathParameter(entryId, { name: 'entryId', style: 'simple', explode: false })}/pin`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST', ...(body !== undefined ? { body, contentType: 'application/json' } : {}), sdkworkUnwrapKind: 'item' });
    }
    /** Community entries.delete */
    async delete(entryId, requestOptions) {
        return this.client.request(backendApiPath(`/community/entries/${serializePathParameter(entryId, { name: 'entryId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'DELETE' });
    }
}
export class CommunityCirclesApi {
    client;
    constructor(client) {
        this.client = client;
    }
    /** Community circles.update */
    async update(categoryId, body, requestOptions) {
        return this.client.request(backendApiPath(`/community/circles/${serializePathParameter(categoryId, { name: 'categoryId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'PATCH', body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
    }
}
export class CommunityCategoriesManagementApi {
    client;
    constructor(client) {
        this.client = client;
    }
    /** Community categories.management.list */
    async list(requestOptions) {
        return this.client.request(backendApiPath(`/community/categories`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET', sdkworkUnwrapKind: 'page' });
    }
}
export class CommunityCategoriesApi {
    client;
    management;
    constructor(client) {
        this.client = client;
        this.management = new CommunityCategoriesManagementApi(client);
    }
    /** Community categories.create */
    async create(body, requestOptions) {
        return this.client.request(backendApiPath(`/community/categories`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST', body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
    }
    /** Community categories.update */
    async update(categoryId, body, requestOptions) {
        return this.client.request(backendApiPath(`/community/categories/${serializePathParameter(categoryId, { name: 'categoryId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'PATCH', body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
    }
    /** Community categories.delete */
    async delete(categoryId, requestOptions) {
        return this.client.request(backendApiPath(`/community/categories/${serializePathParameter(categoryId, { name: 'categoryId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'DELETE' });
    }
}
export class CommunityApi {
    categories;
    circles;
    entries;
    moderation;
    recommendations;
    members;
    groups;
    tiers;
    constructor(client) {
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
export function createCommunityApi(client) {
    return new CommunityApi(client);
}
function appendQueryString(path, rawQueryString) {
    const query = rawQueryString.replace(/^\?+/, '');
    if (!query) {
        return path;
    }
    return path.includes('?') ? `${path}&${query}` : `${path}?${query}`;
}
function serializePathParameter(value, spec) {
    if (value === undefined || value === null) {
        return '';
    }
    const style = spec.style || 'simple';
    if (Array.isArray(value)) {
        return serializePathArray(spec.name, value, style, spec.explode);
    }
    if (typeof value === 'object') {
        return serializePathObject(spec.name, value, style, spec.explode);
    }
    return pathPrefix(spec.name, style, false) + encodePathValue(serializePathPrimitive(value));
}
function serializePathArray(name, values, style, explode) {
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
function serializePathObject(name, value, style, explode) {
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
function pathPrefix(name, style, _objectValue) {
    if (style === 'label')
        return '.';
    if (style === 'matrix')
        return `;${name}`;
    return '';
}
function encodePathValue(value) {
    return encodeURIComponent(value);
}
function serializePathPrimitive(value) {
    if (value instanceof Date) {
        return value.toISOString();
    }
    if (typeof value === 'object') {
        return JSON.stringify(value);
    }
    return String(value);
}
function buildQueryString(parameters) {
    const pairs = [];
    for (const parameter of parameters) {
        appendSerializedParameter(pairs, parameter);
    }
    return pairs.join('&');
}
function appendSerializedParameter(pairs, parameter) {
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
        appendObjectParameter(pairs, parameter.name, parameter.value, style, parameter.explode, parameter.allowReserved);
        return;
    }
    pairs.push(`${encodeQueryComponent(parameter.name)}=${encodeQueryValue(serializePrimitive(parameter.value), parameter.allowReserved)}`);
}
function appendArrayParameter(pairs, name, value, style, explode, allowReserved) {
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
function appendObjectParameter(pairs, name, value, style, explode, allowReserved) {
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
function appendDeepObjectParameter(pairs, name, value, allowReserved) {
    if (!value || typeof value !== 'object' || Array.isArray(value)) {
        pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(serializePrimitive(value), allowReserved)}`);
        return;
    }
    for (const [key, entryValue] of Object.entries(value)) {
        if (entryValue === undefined || entryValue === null) {
            continue;
        }
        pairs.push(`${encodeQueryComponent(`${name}[${key}]`)}=${encodeQueryValue(serializePrimitive(entryValue), allowReserved)}`);
    }
}
function serializePrimitive(value) {
    if (value instanceof Date) {
        return value.toISOString();
    }
    if (typeof value === 'object') {
        return JSON.stringify(value);
    }
    return String(value);
}
function encodeQueryComponent(value) {
    return encodeURIComponent(value);
}
function encodeQueryValue(value, allowReserved) {
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
//# sourceMappingURL=community.js.map