import 'dart:convert';
import '../http/client.dart';
import '../models.dart';

import 'paths.dart';
import 'response_helpers.dart';


class CommunityApi {
  final HttpClient _client;

  CommunityApi(this._client);

  /// Community categories.list
  Future<SdkWorkListResponse?> categoriesList() async {
    final response = await _client.get(ApiPaths.appPath('/community/categories'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkListResponse.fromJson(map);
    })();
  }

  /// Community categories.create
  Future<SdkWorkResourceResponse?> categoriesCreate(CommunityCircleCommand body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.appPath('/community/categories'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkResourceResponse.fromJson(map);
    })();
  }

  /// Community categories.retrieve
  Future<SdkWorkResourceResponse?> categoriesRetrieve(String categoryId) async {
    final response = await _client.get(ApiPaths.appPath('/community/categories/${serializePathParameter(categoryId, const PathParameterSpec('categoryId', 'simple', false))}'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkResourceResponse.fromJson(map);
    })();
  }

  /// Community categories.update
  Future<SdkWorkResourceResponse?> categoriesUpdate(String categoryId, CommunityCircleCommand body) async {
    final payload = body.toJson();
    final response = await _client.patch(ApiPaths.appPath('/community/categories/${serializePathParameter(categoryId, const PathParameterSpec('categoryId', 'simple', false))}'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkResourceResponse.fromJson(map);
    })();
  }

  /// Community categories.delete
  Future<void> categoriesDelete(String categoryId) async {
    await _client.delete(ApiPaths.appPath('/community/categories/${serializePathParameter(categoryId, const PathParameterSpec('categoryId', 'simple', false))}'));
  }

  /// Community categories.join
  Future<SdkWorkResourceResponse?> categoriesJoin(String categoryId) async {
    final response = await _client.post(ApiPaths.appPath('/community/categories/${serializePathParameter(categoryId, const PathParameterSpec('categoryId', 'simple', false))}/join'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkResourceResponse.fromJson(map);
    })();
  }

  /// Community members.list
  Future<SdkWorkListResponse?> membersList(String categoryId) async {
    final response = await _client.get(ApiPaths.appPath('/community/categories/${serializePathParameter(categoryId, const PathParameterSpec('categoryId', 'simple', false))}/members'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkListResponse.fromJson(map);
    })();
  }

  /// Community members.current
  Future<MembersRetrieveResponse?> membersRetrieve(String categoryId) async {
    final response = await _client.get(ApiPaths.appPath('/community/categories/${serializePathParameter(categoryId, const PathParameterSpec('categoryId', 'simple', false))}/members/current'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MembersRetrieveResponse.fromJson(map);
    })();
  }

  /// Community members.update
  Future<SdkWorkResourceResponse?> membersUpdate(String categoryId, String memberId, CommunityMemberPatchCommand body) async {
    final payload = body.toJson();
    final response = await _client.patch(ApiPaths.appPath('/community/categories/${serializePathParameter(categoryId, const PathParameterSpec('categoryId', 'simple', false))}/members/${serializePathParameter(memberId, const PathParameterSpec('memberId', 'simple', false))}'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkResourceResponse.fromJson(map);
    })();
  }

  /// Community members.remove
  Future<void> membersDelete(String categoryId, String memberId) async {
    await _client.delete(ApiPaths.appPath('/community/categories/${serializePathParameter(categoryId, const PathParameterSpec('categoryId', 'simple', false))}/members/${serializePathParameter(memberId, const PathParameterSpec('memberId', 'simple', false))}'));
  }

  /// Community members.activate
  Future<SdkWorkResourceResponse?> membersActivate(String categoryId, CommunityActivateMembershipCommand body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.appPath('/community/categories/${serializePathParameter(categoryId, const PathParameterSpec('categoryId', 'simple', false))}/members/activate'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkResourceResponse.fromJson(map);
    })();
  }

  /// Community tiers.list
  Future<SdkWorkListResponse?> tiersList(String categoryId, [bool? includeDisabled]) async {
    final query = buildQueryString([
      QueryParameterSpec('include_disabled', includeDisabled, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.appPath('/community/categories/${serializePathParameter(categoryId, const PathParameterSpec('categoryId', 'simple', false))}/tiers'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkListResponse.fromJson(map);
    })();
  }

  /// Community tiers.create
  Future<SdkWorkResourceResponse?> tiersCreate(String categoryId, CommunityTierCommand body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.appPath('/community/categories/${serializePathParameter(categoryId, const PathParameterSpec('categoryId', 'simple', false))}/tiers'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkResourceResponse.fromJson(map);
    })();
  }

  /// Community tiers.update
  Future<SdkWorkResourceResponse?> tiersUpdate(String categoryId, String tierId, CommunityTierCommand body) async {
    final payload = body.toJson();
    final response = await _client.patch(ApiPaths.appPath('/community/categories/${serializePathParameter(categoryId, const PathParameterSpec('categoryId', 'simple', false))}/tiers/${serializePathParameter(tierId, const PathParameterSpec('tierId', 'simple', false))}'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkResourceResponse.fromJson(map);
    })();
  }

  /// Community tiers.delete
  Future<void> tiersDelete(String categoryId, String tierId) async {
    await _client.delete(ApiPaths.appPath('/community/categories/${serializePathParameter(categoryId, const PathParameterSpec('categoryId', 'simple', false))}/tiers/${serializePathParameter(tierId, const PathParameterSpec('tierId', 'simple', false))}'));
  }

  /// Community tiers.publish
  Future<SdkWorkResourceResponse?> tiersPublish(String categoryId, String tierId) async {
    final response = await _client.post(ApiPaths.appPath('/community/categories/${serializePathParameter(categoryId, const PathParameterSpec('categoryId', 'simple', false))}/tiers/${serializePathParameter(tierId, const PathParameterSpec('tierId', 'simple', false))}/publish'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkResourceResponse.fromJson(map);
    })();
  }

  /// Community tiers.unpublish
  Future<SdkWorkResourceResponse?> tiersUnpublish(String categoryId, String tierId) async {
    final response = await _client.post(ApiPaths.appPath('/community/categories/${serializePathParameter(categoryId, const PathParameterSpec('categoryId', 'simple', false))}/tiers/${serializePathParameter(tierId, const PathParameterSpec('tierId', 'simple', false))}/unpublish'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkResourceResponse.fromJson(map);
    })();
  }

  /// Community groups.list
  Future<SdkWorkListResponse?> groupsList(String categoryId) async {
    final response = await _client.get(ApiPaths.appPath('/community/categories/${serializePathParameter(categoryId, const PathParameterSpec('categoryId', 'simple', false))}/groups'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkListResponse.fromJson(map);
    })();
  }

  /// Community groups.create
  Future<SdkWorkResourceResponse?> groupsCreate(String categoryId, CommunityGroupCommand body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.appPath('/community/categories/${serializePathParameter(categoryId, const PathParameterSpec('categoryId', 'simple', false))}/groups'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkResourceResponse.fromJson(map);
    })();
  }

  /// Community groups.update
  Future<SdkWorkResourceResponse?> groupsUpdate(String categoryId, String groupId, CommunityGroupCommand body) async {
    final payload = body.toJson();
    final response = await _client.patch(ApiPaths.appPath('/community/categories/${serializePathParameter(categoryId, const PathParameterSpec('categoryId', 'simple', false))}/groups/${serializePathParameter(groupId, const PathParameterSpec('groupId', 'simple', false))}'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkResourceResponse.fromJson(map);
    })();
  }

  /// Community groups.remove
  Future<void> groupsDelete(String categoryId, String groupId) async {
    await _client.delete(ApiPaths.appPath('/community/categories/${serializePathParameter(categoryId, const PathParameterSpec('categoryId', 'simple', false))}/groups/${serializePathParameter(groupId, const PathParameterSpec('groupId', 'simple', false))}'));
  }

  /// Community feed.list (deprecated: use the standard feeds stream system)
  Future<SdkWorkListResponse?> feedList([String? categoryId, String? kind, String? q, String? reviewState, String? tag, int? page, int? pageSize]) async {
    final query = buildQueryString([
      QueryParameterSpec('category_id', categoryId, 'form', true, false, null),
      QueryParameterSpec('kind', kind, 'form', true, false, null),
      QueryParameterSpec('q', q, 'form', true, false, null),
      QueryParameterSpec('review_state', reviewState, 'form', true, false, null),
      QueryParameterSpec('tag', tag, 'form', true, false, null),
      QueryParameterSpec('page', page, 'form', true, false, null),
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null)
    ]);
    final response = await _client.request('GET', ApiPaths.appendQueryString(ApiPaths.appPath('/community/feed'), query), skipAuth: true);
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkListResponse.fromJson(map);
    })();
  }

  /// Community entries.retrieve
  Future<SdkWorkResourceResponse?> entriesRetrieve(String entryId) async {
    final response = await _client.get(ApiPaths.appPath('/community/entries/${serializePathParameter(entryId, const PathParameterSpec('entryId', 'simple', false))}'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkResourceResponse.fromJson(map);
    })();
  }

  /// Community entries.update
  Future<SdkWorkResourceResponse?> entriesUpdate(String entryId, CommunityEntryCommand body) async {
    final payload = body.toJson();
    final response = await _client.patch(ApiPaths.appPath('/community/entries/${serializePathParameter(entryId, const PathParameterSpec('entryId', 'simple', false))}'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkResourceResponse.fromJson(map);
    })();
  }

  /// Community entries.delete
  Future<void> entriesDelete(String entryId) async {
    await _client.delete(ApiPaths.appPath('/community/entries/${serializePathParameter(entryId, const PathParameterSpec('entryId', 'simple', false))}'));
  }

  /// Community entries.recommendations.list
  Future<SdkWorkListResponse?> entriesRecommendationsList(String entryId) async {
    final response = await _client.get(ApiPaths.appPath('/community/entries/${serializePathParameter(entryId, const PathParameterSpec('entryId', 'simple', false))}/recommendations'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkListResponse.fromJson(map);
    })();
  }

  /// Community entries.create
  Future<SdkWorkResourceResponse?> entriesCreate(CommunityEntryCommand body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.appPath('/community/entries'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkResourceResponse.fromJson(map);
    })();
  }

  /// Community entries.publicationReadiness.retrieve
  Future<SdkWorkResourceResponse?> entriesPublicationReadinessRetrieve(String entryId) async {
    final response = await _client.get(ApiPaths.appPath('/community/entries/${serializePathParameter(entryId, const PathParameterSpec('entryId', 'simple', false))}/publication_readiness'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkResourceResponse.fromJson(map);
    })();
  }

  /// Community reactions.set
  Future<SdkWorkResourceResponse?> reactionsCreate(String entryId, CommunityReactionCommand body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.appPath('/community/entries/${serializePathParameter(entryId, const PathParameterSpec('entryId', 'simple', false))}/reactions'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkResourceResponse.fromJson(map);
    })();
  }

  /// Community comments.list
  Future<SdkWorkListResponse?> commentsList(String entryId) async {
    final response = await _client.get(ApiPaths.appPath('/community/entries/${serializePathParameter(entryId, const PathParameterSpec('entryId', 'simple', false))}/comments'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkListResponse.fromJson(map);
    })();
  }

  /// Community comments.create
  Future<SdkWorkResourceResponse?> commentsCreate(String entryId, CommunityCommentCommand body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.appPath('/community/entries/${serializePathParameter(entryId, const PathParameterSpec('entryId', 'simple', false))}/comments'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SdkWorkResourceResponse.fromJson(map);
    })();
  }
}

class PathParameterSpec {
  final String name;
  final String style;
  final bool explode;

  const PathParameterSpec(this.name, this.style, this.explode);
}

String serializePathParameter(dynamic value, PathParameterSpec spec) {
  if (value == null) return '';
  final style = spec.style.trim().isEmpty ? 'simple' : spec.style;
  if (value is Iterable) {
    return serializePathArray(spec.name, value, style, spec.explode);
  }
  if (value is Map) {
    return serializePathObject(spec.name, value, style, spec.explode);
  }
  return pathPrimitivePrefix(spec.name, style) + Uri.encodeComponent(value.toString());
}

String serializePathArray(String name, Iterable values, String style, bool explode) {
  final serialized = values.where((item) => item != null).map((item) => Uri.encodeComponent(item.toString())).toList();
  if (serialized.isEmpty) return pathPrefix(name, style);
  if (style == 'matrix') {
    if (explode) {
      return serialized.map((item) => ';$name=$item').join();
    }
    return ';$name=${serialized.join(',')}';
  }
  final separator = explode ? '.' : ',';
  return pathPrefix(name, style) + serialized.join(separator);
}

String serializePathObject(String name, Map values, String style, bool explode) {
  final entries = <String>[];
  final exploded = <String>[];
  values.forEach((key, value) {
    if (value == null) return;
    final escapedKey = Uri.encodeComponent(key.toString());
    final escapedValue = Uri.encodeComponent(value.toString());
    if (explode) {
      if (style == 'matrix') {
        exploded.add(';$escapedKey=$escapedValue');
      } else {
        exploded.add('$escapedKey=$escapedValue');
      }
    } else {
      entries.add(escapedKey);
      entries.add(escapedValue);
    }
  });
  if (style == 'matrix') {
    if (explode) return exploded.join();
    return ';$name=${entries.join(',')}';
  }
  if (explode) {
    final separator = style == 'label' ? '.' : ',';
    return pathPrefix(name, style) + exploded.join(separator);
  }
  return pathPrefix(name, style) + entries.join(',');
}

String pathPrefix(String name, String style) {
  if (style == 'label') return '.';
  if (style == 'matrix') return ';$name';
  return '';
}

String pathPrimitivePrefix(String name, String style) {
  return style == 'matrix' ? ';$name=' : pathPrefix(name, style);
}
class QueryParameterSpec {
  final String name;
  final dynamic value;
  final String style;
  final bool explode;
  final bool allowReserved;
  final String? contentType;

  const QueryParameterSpec(
    this.name,
    this.value,
    this.style,
    this.explode,
    this.allowReserved,
    this.contentType,
  );
}

String buildQueryString(List<QueryParameterSpec> parameters) {
  final pairs = <String>[];
  for (final parameter in parameters) {
    appendSerializedParameter(pairs, parameter);
  }
  return pairs.join('&');
}

void appendSerializedParameter(List<String> pairs, QueryParameterSpec parameter) {
  final value = parameter.value;
  if (value == null) return;

  final contentType = parameter.contentType;
  if (contentType != null && contentType.trim().isNotEmpty) {
    pairs.add('${urlEncode(parameter.name)}=${encodeQueryValue(jsonEncode(value), parameter.allowReserved)}');
    return;
  }

  final style = parameter.style.trim().isEmpty ? 'form' : parameter.style;
  if (style == 'deepObject' && value is Map) {
    appendDeepObjectParameter(pairs, parameter.name, value, parameter.allowReserved);
    return;
  }
  if (value is Iterable) {
    appendArrayParameter(pairs, parameter.name, value, style, parameter.explode, parameter.allowReserved);
    return;
  }
  if (value is Map) {
    appendObjectParameter(pairs, parameter.name, value, style, parameter.explode, parameter.allowReserved);
    return;
  }
  pairs.add('${urlEncode(parameter.name)}=${encodeQueryValue(value.toString(), parameter.allowReserved)}');
}

void appendArrayParameter(
  List<String> pairs,
  String name,
  Iterable values,
  String style,
  bool explode,
  bool allowReserved,
) {
  final serialized = values.where((item) => item != null).map((item) => item.toString()).toList();
  if (serialized.isEmpty) return;
  if (style == 'form' && explode) {
    for (final item in serialized) {
      pairs.add('${urlEncode(name)}=${encodeQueryValue(item, allowReserved)}');
    }
    return;
  }
  pairs.add('${urlEncode(name)}=${encodeQueryValue(serialized.join(','), allowReserved)}');
}

void appendObjectParameter(
  List<String> pairs,
  String name,
  Map values,
  String style,
  bool explode,
  bool allowReserved,
) {
  final serialized = <String>[];
  values.forEach((key, value) {
    if (value == null) return;
    if (style == 'form' && explode) {
      pairs.add('${urlEncode(key.toString())}=${encodeQueryValue(value.toString(), allowReserved)}');
      return;
    }
    serialized.add(key.toString());
    serialized.add(value.toString());
  });
  if (serialized.isNotEmpty) {
    pairs.add('${urlEncode(name)}=${encodeQueryValue(serialized.join(','), allowReserved)}');
  }
}

void appendDeepObjectParameter(List<String> pairs, String name, Map values, bool allowReserved) {
  values.forEach((key, value) {
    if (value != null) {
      pairs.add('${urlEncode('$name[$key]')}=${encodeQueryValue(value.toString(), allowReserved)}');
    }
  });
}

String encodeQueryValue(String value, bool allowReserved) {
  var encoded = urlEncode(value);
  if (!allowReserved) return encoded;
  const replacements = <String, String>{
    '%3A': ':',
    '%2F': '/',
    '%3F': '?',
    '%23': '#',
    '%5B': '[',
    '%5D': ']',
    '%40': '@',
    '%21': '!',
    '%24': r'$',
    '%26': '&',
    '%27': "'",
    '%28': '(',
    '%29': ')',
    '%2A': '*',
    '%2B': '+',
    '%2C': ',',
    '%3B': ';',
    '%3D': '=',
  };
  replacements.forEach((escaped, reserved) {
    encoded = encoded.replaceAll(escaped, reserved);
  });
  return encoded;
}

String urlEncode(String value) => Uri.encodeQueryComponent(value);
