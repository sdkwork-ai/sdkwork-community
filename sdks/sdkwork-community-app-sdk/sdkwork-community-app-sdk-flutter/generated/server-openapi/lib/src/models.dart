Map<String, dynamic>? _sdkworkAsMap(dynamic value) {
  if (value is Map<String, dynamic>) {
    return value;
  }
  if (value is Map) {
    return value.map((key, item) => MapEntry(key.toString(), item));
  }
  return null;
}

List<dynamic>? _sdkworkAsList(dynamic value) {
  return value is List ? value : null;
}

class SdkWorkApiResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SdkWorkApiResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SdkWorkApiResponse.fromJson(Map<String, dynamic> json) {
    return SdkWorkApiResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SdkWorkApiResponse.code is required');
        }
        return value;
      })(),
      data: json['data'],
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SdkWorkApiResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SdkWorkResourceData {
  final Map<String, dynamic> item;

  SdkWorkResourceData({
    required this.item
  });

  factory SdkWorkResourceData.fromJson(Map<String, dynamic> json) {
    return SdkWorkResourceData(
      item: (() {
        final map = _sdkworkAsMap(json['item']);
        if (map == null) {
          throw FormatException('SdkWorkResourceData.item is required');
        }
        return map;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'item': item,
    };
  }
}

class SdkWorkPageData {
  final List<Map<String, dynamic>> items;
  final PageInfo pageInfo;

  SdkWorkPageData({
    required this.items,
    required this.pageInfo
  });

  factory SdkWorkPageData.fromJson(Map<String, dynamic> json) {
    return SdkWorkPageData(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('SdkWorkPageData.items is required');
        }
        return list
            .map((item) => _sdkworkAsMap(item))
            .whereType<Map<String, dynamic>>()
            .toList();
      })(),
      pageInfo: (() {
        final map = _sdkworkAsMap(json['pageInfo']);
        if (map == null) {
          throw FormatException('SdkWorkPageData.pageInfo is required');
        }
        return PageInfo.fromJson(map);
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item).toList(),
      'pageInfo': pageInfo.toJson(),
    };
  }
}

class SdkWorkCommandData {
  final bool accepted;
  final String? resourceId;
  final String? status;

  SdkWorkCommandData({
    required this.accepted,
    this.resourceId,
    this.status
  });

  factory SdkWorkCommandData.fromJson(Map<String, dynamic> json) {
    return SdkWorkCommandData(
      accepted: (() {
        final value = json['accepted'];
        if (value is! bool) {
          throw FormatException('SdkWorkCommandData.accepted is required');
        }
        return value;
      })(),
      resourceId: json['resourceId']?.toString(),
      status: json['status']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'accepted': accepted,
      'resourceId': resourceId,
      'status': status,
    };
  }
}

class PageInfo {
  final String mode;
  final int? page;
  final int? pageSize;
  final String? totalItems;
  final int? totalPages;
  final String? nextCursor;
  final bool? hasMore;

  PageInfo({
    required this.mode,
    this.page,
    this.pageSize,
    this.totalItems,
    this.totalPages,
    this.nextCursor,
    this.hasMore
  });

  factory PageInfo.fromJson(Map<String, dynamic> json) {
    return PageInfo(
      mode: (() {
        final value = json['mode']?.toString();
        if (value == null) {
          throw FormatException('PageInfo.mode is required');
        }
        return value;
      })(),
      page: json['page'] is int ? json['page'] : null,
      pageSize: json['pageSize'] is int ? json['pageSize'] : null,
      totalItems: json['totalItems']?.toString(),
      totalPages: json['totalPages'] is int ? json['totalPages'] : null,
      nextCursor: json['nextCursor']?.toString(),
      hasMore: json['hasMore'] is bool ? json['hasMore'] : null
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'mode': mode,
      'page': page,
      'pageSize': pageSize,
      'totalItems': totalItems,
      'totalPages': totalPages,
      'nextCursor': nextCursor,
      'hasMore': hasMore,
    };
  }
}

class ProblemDetail {
  final String type;
  final String title;
  final int status;
  final String? detail;
  final String? instance;
  final int code;
  final String traceId;
  final String? i18nKey;
  final String? locale;
  final List<FieldError>? errors;

  ProblemDetail({
    required this.type,
    required this.title,
    required this.status,
    this.detail,
    this.instance,
    required this.code,
    required this.traceId,
    this.i18nKey,
    this.locale,
    this.errors
  });

  factory ProblemDetail.fromJson(Map<String, dynamic> json) {
    return ProblemDetail(
      type: (() {
        final value = json['type']?.toString();
        if (value == null) {
          throw FormatException('ProblemDetail.type is required');
        }
        return value;
      })(),
      title: (() {
        final value = json['title']?.toString();
        if (value == null) {
          throw FormatException('ProblemDetail.title is required');
        }
        return value;
      })(),
      status: (() {
        final value = json['status'];
        if (value is! int) {
          throw FormatException('ProblemDetail.status is required');
        }
        return value;
      })(),
      detail: json['detail']?.toString(),
      instance: json['instance']?.toString(),
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('ProblemDetail.code is required');
        }
        return value;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('ProblemDetail.traceId is required');
        }
        return value;
      })(),
      i18nKey: json['i18nKey']?.toString(),
      locale: json['locale']?.toString(),
      errors: (() {
        final list = _sdkworkAsList(json['errors']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : FieldError.fromJson(map);
      })())
            .whereType<FieldError>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'type': type,
      'title': title,
      'status': status,
      'detail': detail,
      'instance': instance,
      'code': code,
      'traceId': traceId,
      'i18nKey': i18nKey,
      'locale': locale,
      'errors': errors?.map((item) => item.toJson()).toList(),
    };
  }
}

class FieldError {
  final String field;
  final String message;
  final int? code;
  final String? i18nKey;
  final Map<String, dynamic>? params;

  FieldError({
    required this.field,
    required this.message,
    this.code,
    this.i18nKey,
    this.params
  });

  factory FieldError.fromJson(Map<String, dynamic> json) {
    return FieldError(
      field: (() {
        final value = json['field']?.toString();
        if (value == null) {
          throw FormatException('FieldError.field is required');
        }
        return value;
      })(),
      message: (() {
        final value = json['message']?.toString();
        if (value == null) {
          throw FormatException('FieldError.message is required');
        }
        return value;
      })(),
      code: json['code'] is int ? json['code'] : null,
      i18nKey: json['i18nKey']?.toString(),
      params: (() {
        final map = _sdkworkAsMap(json['params']);
        if (map == null) {
          return null;
        }
        final result = <String, String>{};
        map.forEach((key, item) {
          final deserialized = item?.toString();
          if (deserialized is String) {
            result[key] = deserialized;
          }
        });
        return result;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'field': field,
      'message': message,
      'code': code,
      'i18nKey': i18nKey,
      'params': params?.map((key, item) => MapEntry(key, item)),
    };
  }
}

class SdkWorkResourceResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SdkWorkResourceResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SdkWorkResourceResponse.fromJson(Map<String, dynamic> json) {
    return SdkWorkResourceResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SdkWorkResourceResponse.code is required');
        }
        return value;
      })(),
      data: json['data'],
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SdkWorkResourceResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SdkWorkListResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SdkWorkListResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SdkWorkListResponse.fromJson(Map<String, dynamic> json) {
    return SdkWorkListResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SdkWorkListResponse.code is required');
        }
        return value;
      })(),
      data: json['data'],
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SdkWorkListResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SdkWorkCommandResponse {
  final int code;
  final dynamic data;
  final String traceId;

  SdkWorkCommandResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory SdkWorkCommandResponse.fromJson(Map<String, dynamic> json) {
    return SdkWorkCommandResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('SdkWorkCommandResponse.code is required');
        }
        return value;
      })(),
      data: json['data'],
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('SdkWorkCommandResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class CommunityCategory {
  final String id;
  final String tenantId;
  final String slug;
  final String title;
  final String? description;
  final String? coverImage;
  final String? avatar;
  final String? ownerId;
  final String memberCount;
  final String? memberLimit;
  final String postCount;
  final bool isPaid;
  final double? price;
  final double? revenueRaised;
  final double? revenueTarget;
  final List<String> tags;
  final List<String>? tabs;
  final int priority;
  final bool enabled;
  final bool isJoined;

  CommunityCategory({
    required this.id,
    required this.tenantId,
    required this.slug,
    required this.title,
    this.description,
    this.coverImage,
    this.avatar,
    this.ownerId,
    required this.memberCount,
    this.memberLimit,
    required this.postCount,
    required this.isPaid,
    this.price,
    this.revenueRaised,
    this.revenueTarget,
    required this.tags,
    this.tabs,
    required this.priority,
    required this.enabled,
    required this.isJoined
  });

  factory CommunityCategory.fromJson(Map<String, dynamic> json) {
    return CommunityCategory(
      id: (() {
        final value = json['id']?.toString();
        if (value == null) {
          throw FormatException('CommunityCategory.id is required');
        }
        return value;
      })(),
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('CommunityCategory.tenantId is required');
        }
        return value;
      })(),
      slug: (() {
        final value = json['slug']?.toString();
        if (value == null) {
          throw FormatException('CommunityCategory.slug is required');
        }
        return value;
      })(),
      title: (() {
        final value = json['title']?.toString();
        if (value == null) {
          throw FormatException('CommunityCategory.title is required');
        }
        return value;
      })(),
      description: json['description']?.toString(),
      coverImage: json['coverImage']?.toString(),
      avatar: json['avatar']?.toString(),
      ownerId: json['ownerId']?.toString(),
      memberCount: (() {
        final value = json['memberCount']?.toString();
        if (value == null) {
          throw FormatException('CommunityCategory.memberCount is required');
        }
        return value;
      })(),
      memberLimit: json['memberLimit']?.toString(),
      postCount: (() {
        final value = json['postCount']?.toString();
        if (value == null) {
          throw FormatException('CommunityCategory.postCount is required');
        }
        return value;
      })(),
      isPaid: (() {
        final value = json['isPaid'];
        if (value is! bool) {
          throw FormatException('CommunityCategory.isPaid is required');
        }
        return value;
      })(),
      price: json['price'] is num ? json['price'].toDouble() : null,
      revenueRaised: json['revenueRaised'] is num ? json['revenueRaised'].toDouble() : null,
      revenueTarget: json['revenueTarget'] is num ? json['revenueTarget'].toDouble() : null,
      tags: (() {
        final list = _sdkworkAsList(json['tags']);
        if (list == null) {
          throw FormatException('CommunityCategory.tags is required');
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      tabs: (() {
        final list = _sdkworkAsList(json['tabs']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      priority: (() {
        final value = json['priority'];
        if (value is! int) {
          throw FormatException('CommunityCategory.priority is required');
        }
        return value;
      })(),
      enabled: (() {
        final value = json['enabled'];
        if (value is! bool) {
          throw FormatException('CommunityCategory.enabled is required');
        }
        return value;
      })(),
      isJoined: (() {
        final value = json['isJoined'];
        if (value is! bool) {
          throw FormatException('CommunityCategory.isJoined is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'id': id,
      'tenantId': tenantId,
      'slug': slug,
      'title': title,
      'description': description,
      'coverImage': coverImage,
      'avatar': avatar,
      'ownerId': ownerId,
      'memberCount': memberCount,
      'memberLimit': memberLimit,
      'postCount': postCount,
      'isPaid': isPaid,
      'price': price,
      'revenueRaised': revenueRaised,
      'revenueTarget': revenueTarget,
      'tags': tags.map((item) => item).toList(),
      'tabs': tabs?.map((item) => item).toList(),
      'priority': priority,
      'enabled': enabled,
      'isJoined': isJoined,
    };
  }
}

class CommunityCircleCommand {
  final String title;
  final String? description;
  final String? coverImage;
  final String? avatar;
  final bool? isPaid;
  final String? memberLimit;
  final double? price;
  final double? revenueTarget;
  final List<String>? tags;
  final List<String>? tabs;

  CommunityCircleCommand({
    required this.title,
    this.description,
    this.coverImage,
    this.avatar,
    this.isPaid,
    this.memberLimit,
    this.price,
    this.revenueTarget,
    this.tags,
    this.tabs
  });

  factory CommunityCircleCommand.fromJson(Map<String, dynamic> json) {
    return CommunityCircleCommand(
      title: (() {
        final value = json['title']?.toString();
        if (value == null) {
          throw FormatException('CommunityCircleCommand.title is required');
        }
        return value;
      })(),
      description: json['description']?.toString(),
      coverImage: json['coverImage']?.toString(),
      avatar: json['avatar']?.toString(),
      isPaid: json['isPaid'] is bool ? json['isPaid'] : null,
      memberLimit: json['memberLimit']?.toString(),
      price: json['price'] is num ? json['price'].toDouble() : null,
      revenueTarget: json['revenueTarget'] is num ? json['revenueTarget'].toDouble() : null,
      tags: (() {
        final list = _sdkworkAsList(json['tags']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      tabs: (() {
        final list = _sdkworkAsList(json['tabs']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'title': title,
      'description': description,
      'coverImage': coverImage,
      'avatar': avatar,
      'isPaid': isPaid,
      'memberLimit': memberLimit,
      'price': price,
      'revenueTarget': revenueTarget,
      'tags': tags?.map((item) => item).toList(),
      'tabs': tabs?.map((item) => item).toList(),
    };
  }
}

class CommunityMemberResponse {
  final String id;
  final String tenantId;
  final String categoryId;
  final String userId;
  final String userName;
  final String role;
  final String status;
  final String? bio;
  final String? tierId;
  final String? tierName;
  final String? membershipExpiresAt;
  final String? agentLevel;
  final String? lastOrderId;
  final String joinedAt;

  CommunityMemberResponse({
    required this.id,
    required this.tenantId,
    required this.categoryId,
    required this.userId,
    required this.userName,
    required this.role,
    required this.status,
    this.bio,
    this.tierId,
    this.tierName,
    this.membershipExpiresAt,
    this.agentLevel,
    this.lastOrderId,
    required this.joinedAt
  });

  factory CommunityMemberResponse.fromJson(Map<String, dynamic> json) {
    return CommunityMemberResponse(
      id: (() {
        final value = json['id']?.toString();
        if (value == null) {
          throw FormatException('CommunityMemberResponse.id is required');
        }
        return value;
      })(),
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('CommunityMemberResponse.tenantId is required');
        }
        return value;
      })(),
      categoryId: (() {
        final value = json['categoryId']?.toString();
        if (value == null) {
          throw FormatException('CommunityMemberResponse.categoryId is required');
        }
        return value;
      })(),
      userId: (() {
        final value = json['userId']?.toString();
        if (value == null) {
          throw FormatException('CommunityMemberResponse.userId is required');
        }
        return value;
      })(),
      userName: (() {
        final value = json['userName']?.toString();
        if (value == null) {
          throw FormatException('CommunityMemberResponse.userName is required');
        }
        return value;
      })(),
      role: (() {
        final value = json['role']?.toString();
        if (value == null) {
          throw FormatException('CommunityMemberResponse.role is required');
        }
        return value;
      })(),
      status: (() {
        final value = json['status']?.toString();
        if (value == null) {
          throw FormatException('CommunityMemberResponse.status is required');
        }
        return value;
      })(),
      bio: json['bio']?.toString(),
      tierId: json['tierId']?.toString(),
      tierName: json['tierName']?.toString(),
      membershipExpiresAt: json['membershipExpiresAt']?.toString(),
      agentLevel: json['agentLevel']?.toString(),
      lastOrderId: json['lastOrderId']?.toString(),
      joinedAt: (() {
        final value = json['joinedAt']?.toString();
        if (value == null) {
          throw FormatException('CommunityMemberResponse.joinedAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'id': id,
      'tenantId': tenantId,
      'categoryId': categoryId,
      'userId': userId,
      'userName': userName,
      'role': role,
      'status': status,
      'bio': bio,
      'tierId': tierId,
      'tierName': tierName,
      'membershipExpiresAt': membershipExpiresAt,
      'agentLevel': agentLevel,
      'lastOrderId': lastOrderId,
      'joinedAt': joinedAt,
    };
  }
}

class CommunityMemberPatchCommand {
  final String? role;
  final String? status;

  CommunityMemberPatchCommand({
    this.role,
    this.status
  });

  factory CommunityMemberPatchCommand.fromJson(Map<String, dynamic> json) {
    return CommunityMemberPatchCommand(
      role: json['role']?.toString(),
      status: json['status']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'role': role,
      'status': status,
    };
  }
}

class CommunityGroupQr {
  final String url;
  final String? description;

  CommunityGroupQr({
    required this.url,
    this.description
  });

  factory CommunityGroupQr.fromJson(Map<String, dynamic> json) {
    return CommunityGroupQr(
      url: (() {
        final value = json['url']?.toString();
        if (value == null) {
          throw FormatException('CommunityGroupQr.url is required');
        }
        return value;
      })(),
      description: json['description']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'url': url,
      'description': description,
    };
  }
}

class CommunityGroupResponse {
  final String id;
  final String tenantId;
  final String categoryId;
  final String name;
  final String platform;
  final String? description;
  final String memberCount;
  final List<CommunityGroupQr> qrCodes;
  final String createdAt;
  final String updatedAt;

  CommunityGroupResponse({
    required this.id,
    required this.tenantId,
    required this.categoryId,
    required this.name,
    required this.platform,
    this.description,
    required this.memberCount,
    required this.qrCodes,
    required this.createdAt,
    required this.updatedAt
  });

  factory CommunityGroupResponse.fromJson(Map<String, dynamic> json) {
    return CommunityGroupResponse(
      id: (() {
        final value = json['id']?.toString();
        if (value == null) {
          throw FormatException('CommunityGroupResponse.id is required');
        }
        return value;
      })(),
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('CommunityGroupResponse.tenantId is required');
        }
        return value;
      })(),
      categoryId: (() {
        final value = json['categoryId']?.toString();
        if (value == null) {
          throw FormatException('CommunityGroupResponse.categoryId is required');
        }
        return value;
      })(),
      name: (() {
        final value = json['name']?.toString();
        if (value == null) {
          throw FormatException('CommunityGroupResponse.name is required');
        }
        return value;
      })(),
      platform: (() {
        final value = json['platform']?.toString();
        if (value == null) {
          throw FormatException('CommunityGroupResponse.platform is required');
        }
        return value;
      })(),
      description: json['description']?.toString(),
      memberCount: (() {
        final value = json['memberCount']?.toString();
        if (value == null) {
          throw FormatException('CommunityGroupResponse.memberCount is required');
        }
        return value;
      })(),
      qrCodes: (() {
        final list = _sdkworkAsList(json['qrCodes']);
        if (list == null) {
          throw FormatException('CommunityGroupResponse.qrCodes is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : CommunityGroupQr.fromJson(map);
      })())
            .whereType<CommunityGroupQr>()
            .toList();
      })(),
      createdAt: (() {
        final value = json['createdAt']?.toString();
        if (value == null) {
          throw FormatException('CommunityGroupResponse.createdAt is required');
        }
        return value;
      })(),
      updatedAt: (() {
        final value = json['updatedAt']?.toString();
        if (value == null) {
          throw FormatException('CommunityGroupResponse.updatedAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'id': id,
      'tenantId': tenantId,
      'categoryId': categoryId,
      'name': name,
      'platform': platform,
      'description': description,
      'memberCount': memberCount,
      'qrCodes': qrCodes.map((item) => item.toJson()).toList(),
      'createdAt': createdAt,
      'updatedAt': updatedAt,
    };
  }
}

class CommunityGroupCommand {
  final String name;
  final String platform;
  final String? description;
  final String? memberCount;
  final List<CommunityGroupQr>? qrCodes;

  CommunityGroupCommand({
    required this.name,
    required this.platform,
    this.description,
    this.memberCount,
    this.qrCodes
  });

  factory CommunityGroupCommand.fromJson(Map<String, dynamic> json) {
    return CommunityGroupCommand(
      name: (() {
        final value = json['name']?.toString();
        if (value == null) {
          throw FormatException('CommunityGroupCommand.name is required');
        }
        return value;
      })(),
      platform: (() {
        final value = json['platform']?.toString();
        if (value == null) {
          throw FormatException('CommunityGroupCommand.platform is required');
        }
        return value;
      })(),
      description: json['description']?.toString(),
      memberCount: json['memberCount']?.toString(),
      qrCodes: (() {
        final list = _sdkworkAsList(json['qrCodes']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : CommunityGroupQr.fromJson(map);
      })())
            .whereType<CommunityGroupQr>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'name': name,
      'platform': platform,
      'description': description,
      'memberCount': memberCount,
      'qrCodes': qrCodes?.map((item) => item.toJson()).toList(),
    };
  }
}

class CommunityMembershipTier {
  final String id;
  final String tenantId;
  final String categoryId;
  final String name;
  final String? description;
  final double price;
  final String durationDays;
  final double? lifetimePrice;
  final String? lifetimePackageId;
  final List<String> benefits;
  final String? agentLevel;
  final String? catalogPackageId;
  final String sortOrder;
  final bool enabled;

  CommunityMembershipTier({
    required this.id,
    required this.tenantId,
    required this.categoryId,
    required this.name,
    this.description,
    required this.price,
    required this.durationDays,
    this.lifetimePrice,
    this.lifetimePackageId,
    required this.benefits,
    this.agentLevel,
    this.catalogPackageId,
    required this.sortOrder,
    required this.enabled
  });

  factory CommunityMembershipTier.fromJson(Map<String, dynamic> json) {
    return CommunityMembershipTier(
      id: (() {
        final value = json['id']?.toString();
        if (value == null) {
          throw FormatException('CommunityMembershipTier.id is required');
        }
        return value;
      })(),
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('CommunityMembershipTier.tenantId is required');
        }
        return value;
      })(),
      categoryId: (() {
        final value = json['categoryId']?.toString();
        if (value == null) {
          throw FormatException('CommunityMembershipTier.categoryId is required');
        }
        return value;
      })(),
      name: (() {
        final value = json['name']?.toString();
        if (value == null) {
          throw FormatException('CommunityMembershipTier.name is required');
        }
        return value;
      })(),
      description: json['description']?.toString(),
      price: (() {
        final value = json['price'];
        if (value is! num) {
          throw FormatException('CommunityMembershipTier.price is required');
        }
        return value.toDouble();
      })(),
      durationDays: (() {
        final value = json['durationDays']?.toString();
        if (value == null) {
          throw FormatException('CommunityMembershipTier.durationDays is required');
        }
        return value;
      })(),
      lifetimePrice: json['lifetimePrice'] is num ? json['lifetimePrice'].toDouble() : null,
      lifetimePackageId: json['lifetimePackageId']?.toString(),
      benefits: (() {
        final list = _sdkworkAsList(json['benefits']);
        if (list == null) {
          throw FormatException('CommunityMembershipTier.benefits is required');
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      agentLevel: json['agentLevel']?.toString(),
      catalogPackageId: json['catalogPackageId']?.toString(),
      sortOrder: (() {
        final value = json['sortOrder']?.toString();
        if (value == null) {
          throw FormatException('CommunityMembershipTier.sortOrder is required');
        }
        return value;
      })(),
      enabled: (() {
        final value = json['enabled'];
        if (value is! bool) {
          throw FormatException('CommunityMembershipTier.enabled is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'id': id,
      'tenantId': tenantId,
      'categoryId': categoryId,
      'name': name,
      'description': description,
      'price': price,
      'durationDays': durationDays,
      'lifetimePrice': lifetimePrice,
      'lifetimePackageId': lifetimePackageId,
      'benefits': benefits.map((item) => item).toList(),
      'agentLevel': agentLevel,
      'catalogPackageId': catalogPackageId,
      'sortOrder': sortOrder,
      'enabled': enabled,
    };
  }
}

class CommunityTierCommand {
  final String name;
  final String? description;
  final double price;
  final String? durationDays;
  final double? lifetimePrice;
  final List<String>? benefits;
  final String? agentLevel;
  final String? sortOrder;

  CommunityTierCommand({
    required this.name,
    this.description,
    required this.price,
    this.durationDays,
    this.lifetimePrice,
    this.benefits,
    this.agentLevel,
    this.sortOrder
  });

  factory CommunityTierCommand.fromJson(Map<String, dynamic> json) {
    return CommunityTierCommand(
      name: (() {
        final value = json['name']?.toString();
        if (value == null) {
          throw FormatException('CommunityTierCommand.name is required');
        }
        return value;
      })(),
      description: json['description']?.toString(),
      price: (() {
        final value = json['price'];
        if (value is! num) {
          throw FormatException('CommunityTierCommand.price is required');
        }
        return value.toDouble();
      })(),
      durationDays: json['durationDays']?.toString(),
      lifetimePrice: json['lifetimePrice'] is num ? json['lifetimePrice'].toDouble() : null,
      benefits: (() {
        final list = _sdkworkAsList(json['benefits']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      agentLevel: json['agentLevel']?.toString(),
      sortOrder: json['sortOrder']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'name': name,
      'description': description,
      'price': price,
      'durationDays': durationDays,
      'lifetimePrice': lifetimePrice,
      'benefits': benefits?.map((item) => item).toList(),
      'agentLevel': agentLevel,
      'sortOrder': sortOrder,
    };
  }
}

class CommunityActivateMembershipCommand {
  final String orderId;
  final String tierId;

  CommunityActivateMembershipCommand({
    required this.orderId,
    required this.tierId
  });

  factory CommunityActivateMembershipCommand.fromJson(Map<String, dynamic> json) {
    return CommunityActivateMembershipCommand(
      orderId: (() {
        final value = json['orderId']?.toString();
        if (value == null) {
          throw FormatException('CommunityActivateMembershipCommand.orderId is required');
        }
        return value;
      })(),
      tierId: (() {
        final value = json['tierId']?.toString();
        if (value == null) {
          throw FormatException('CommunityActivateMembershipCommand.tierId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'orderId': orderId,
      'tierId': tierId,
    };
  }
}

class CommunityAuthor {
  final String id;
  final String name;
  final String? avatarUrl;

  CommunityAuthor({
    required this.id,
    required this.name,
    this.avatarUrl
  });

  factory CommunityAuthor.fromJson(Map<String, dynamic> json) {
    return CommunityAuthor(
      id: (() {
        final value = json['id']?.toString();
        if (value == null) {
          throw FormatException('CommunityAuthor.id is required');
        }
        return value;
      })(),
      name: (() {
        final value = json['name']?.toString();
        if (value == null) {
          throw FormatException('CommunityAuthor.name is required');
        }
        return value;
      })(),
      avatarUrl: json['avatarUrl']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'id': id,
      'name': name,
      'avatarUrl': avatarUrl,
    };
  }
}

class CommunityStats {
  final int? commentCount;
  final int? reactionCount;
  final int? shareCount;
  final int? viewCount;

  CommunityStats({
    this.commentCount,
    this.reactionCount,
    this.shareCount,
    this.viewCount
  });

  factory CommunityStats.fromJson(Map<String, dynamic> json) {
    return CommunityStats(
      commentCount: json['commentCount'] is int ? json['commentCount'] : null,
      reactionCount: json['reactionCount'] is int ? json['reactionCount'] : null,
      shareCount: json['shareCount'] is int ? json['shareCount'] : null,
      viewCount: json['viewCount'] is int ? json['viewCount'] : null
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'commentCount': commentCount,
      'reactionCount': reactionCount,
      'shareCount': shareCount,
      'viewCount': viewCount,
    };
  }
}

class CommunityEntry {
  final String id;
  final String tenantId;
  final String categoryId;
  final String? categoryLabel;
  final CommunityAuthor author;
  final String slug;
  final String kind;
  final String title;
  final String? excerpt;
  final String? body;
  final String reviewState;
  final bool? isFeatured;
  final bool? isPinned;
  final bool? hasAcceptedAnswer;
  final CommunityStats stats;
  final List<String>? tags;
  final List<String>? media;
  final String? publishedAt;
  final String? lastActivityAt;
  final String? updatedAt;

  CommunityEntry({
    required this.id,
    required this.tenantId,
    required this.categoryId,
    this.categoryLabel,
    required this.author,
    required this.slug,
    required this.kind,
    required this.title,
    this.excerpt,
    this.body,
    required this.reviewState,
    this.isFeatured,
    this.isPinned,
    this.hasAcceptedAnswer,
    required this.stats,
    this.tags,
    this.media,
    this.publishedAt,
    this.lastActivityAt,
    this.updatedAt
  });

  factory CommunityEntry.fromJson(Map<String, dynamic> json) {
    return CommunityEntry(
      id: (() {
        final value = json['id']?.toString();
        if (value == null) {
          throw FormatException('CommunityEntry.id is required');
        }
        return value;
      })(),
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('CommunityEntry.tenantId is required');
        }
        return value;
      })(),
      categoryId: (() {
        final value = json['categoryId']?.toString();
        if (value == null) {
          throw FormatException('CommunityEntry.categoryId is required');
        }
        return value;
      })(),
      categoryLabel: json['categoryLabel']?.toString(),
      author: (() {
        final map = _sdkworkAsMap(json['author']);
        if (map == null) {
          throw FormatException('CommunityEntry.author is required');
        }
        return CommunityAuthor.fromJson(map);
      })(),
      slug: (() {
        final value = json['slug']?.toString();
        if (value == null) {
          throw FormatException('CommunityEntry.slug is required');
        }
        return value;
      })(),
      kind: (() {
        final value = json['kind']?.toString();
        if (value == null) {
          throw FormatException('CommunityEntry.kind is required');
        }
        return value;
      })(),
      title: (() {
        final value = json['title']?.toString();
        if (value == null) {
          throw FormatException('CommunityEntry.title is required');
        }
        return value;
      })(),
      excerpt: json['excerpt']?.toString(),
      body: json['body']?.toString(),
      reviewState: (() {
        final value = json['reviewState']?.toString();
        if (value == null) {
          throw FormatException('CommunityEntry.reviewState is required');
        }
        return value;
      })(),
      isFeatured: json['isFeatured'] is bool ? json['isFeatured'] : null,
      isPinned: json['isPinned'] is bool ? json['isPinned'] : null,
      hasAcceptedAnswer: json['hasAcceptedAnswer'] is bool ? json['hasAcceptedAnswer'] : null,
      stats: (() {
        final map = _sdkworkAsMap(json['stats']);
        if (map == null) {
          throw FormatException('CommunityEntry.stats is required');
        }
        return CommunityStats.fromJson(map);
      })(),
      tags: (() {
        final list = _sdkworkAsList(json['tags']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      media: (() {
        final list = _sdkworkAsList(json['media']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      publishedAt: json['publishedAt']?.toString(),
      lastActivityAt: json['lastActivityAt']?.toString(),
      updatedAt: json['updatedAt']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'id': id,
      'tenantId': tenantId,
      'categoryId': categoryId,
      'categoryLabel': categoryLabel,
      'author': author.toJson(),
      'slug': slug,
      'kind': kind,
      'title': title,
      'excerpt': excerpt,
      'body': body,
      'reviewState': reviewState,
      'isFeatured': isFeatured,
      'isPinned': isPinned,
      'hasAcceptedAnswer': hasAcceptedAnswer,
      'stats': stats.toJson(),
      'tags': tags?.map((item) => item).toList(),
      'media': media?.map((item) => item).toList(),
      'publishedAt': publishedAt,
      'lastActivityAt': lastActivityAt,
      'updatedAt': updatedAt,
    };
  }
}

class CommunityComment {
  final String id;
  final String tenantId;
  final String entryId;
  final CommunityAuthor author;
  final String body;
  final String reviewState;
  final bool? isAcceptedAnswer;
  final String createdAt;
  final String? updatedAt;

  CommunityComment({
    required this.id,
    required this.tenantId,
    required this.entryId,
    required this.author,
    required this.body,
    required this.reviewState,
    this.isAcceptedAnswer,
    required this.createdAt,
    this.updatedAt
  });

  factory CommunityComment.fromJson(Map<String, dynamic> json) {
    return CommunityComment(
      id: (() {
        final value = json['id']?.toString();
        if (value == null) {
          throw FormatException('CommunityComment.id is required');
        }
        return value;
      })(),
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('CommunityComment.tenantId is required');
        }
        return value;
      })(),
      entryId: (() {
        final value = json['entryId']?.toString();
        if (value == null) {
          throw FormatException('CommunityComment.entryId is required');
        }
        return value;
      })(),
      author: (() {
        final map = _sdkworkAsMap(json['author']);
        if (map == null) {
          throw FormatException('CommunityComment.author is required');
        }
        return CommunityAuthor.fromJson(map);
      })(),
      body: (() {
        final value = json['body']?.toString();
        if (value == null) {
          throw FormatException('CommunityComment.body is required');
        }
        return value;
      })(),
      reviewState: (() {
        final value = json['reviewState']?.toString();
        if (value == null) {
          throw FormatException('CommunityComment.reviewState is required');
        }
        return value;
      })(),
      isAcceptedAnswer: json['isAcceptedAnswer'] is bool ? json['isAcceptedAnswer'] : null,
      createdAt: (() {
        final value = json['createdAt']?.toString();
        if (value == null) {
          throw FormatException('CommunityComment.createdAt is required');
        }
        return value;
      })(),
      updatedAt: json['updatedAt']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'id': id,
      'tenantId': tenantId,
      'entryId': entryId,
      'author': author.toJson(),
      'body': body,
      'reviewState': reviewState,
      'isAcceptedAnswer': isAcceptedAnswer,
      'createdAt': createdAt,
      'updatedAt': updatedAt,
    };
  }
}

class CommunityEntryCommand {
  final String categoryId;
  final String kind;
  final String title;
  final String? excerpt;
  final String? body;
  final List<String>? tags;
  final List<String>? media;

  CommunityEntryCommand({
    required this.categoryId,
    required this.kind,
    required this.title,
    this.excerpt,
    this.body,
    this.tags,
    this.media
  });

  factory CommunityEntryCommand.fromJson(Map<String, dynamic> json) {
    return CommunityEntryCommand(
      categoryId: (() {
        final value = json['categoryId']?.toString();
        if (value == null) {
          throw FormatException('CommunityEntryCommand.categoryId is required');
        }
        return value;
      })(),
      kind: (() {
        final value = json['kind']?.toString();
        if (value == null) {
          throw FormatException('CommunityEntryCommand.kind is required');
        }
        return value;
      })(),
      title: (() {
        final value = json['title']?.toString();
        if (value == null) {
          throw FormatException('CommunityEntryCommand.title is required');
        }
        return value;
      })(),
      excerpt: json['excerpt']?.toString(),
      body: json['body']?.toString(),
      tags: (() {
        final list = _sdkworkAsList(json['tags']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      media: (() {
        final list = _sdkworkAsList(json['media']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'categoryId': categoryId,
      'kind': kind,
      'title': title,
      'excerpt': excerpt,
      'body': body,
      'tags': tags?.map((item) => item).toList(),
      'media': media?.map((item) => item).toList(),
    };
  }
}

class CommunityCategoryCommand {
  final String slug;
  final String title;
  final String? description;
  final int? priority;
  final bool? enabled;

  CommunityCategoryCommand({
    required this.slug,
    required this.title,
    this.description,
    this.priority,
    this.enabled
  });

  factory CommunityCategoryCommand.fromJson(Map<String, dynamic> json) {
    return CommunityCategoryCommand(
      slug: (() {
        final value = json['slug']?.toString();
        if (value == null) {
          throw FormatException('CommunityCategoryCommand.slug is required');
        }
        return value;
      })(),
      title: (() {
        final value = json['title']?.toString();
        if (value == null) {
          throw FormatException('CommunityCategoryCommand.title is required');
        }
        return value;
      })(),
      description: json['description']?.toString(),
      priority: json['priority'] is int ? json['priority'] : null,
      enabled: json['enabled'] is bool ? json['enabled'] : null
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'slug': slug,
      'title': title,
      'description': description,
      'priority': priority,
      'enabled': enabled,
    };
  }
}

class CommunityCommentCommand {
  final String body;

  CommunityCommentCommand({
    required this.body
  });

  factory CommunityCommentCommand.fromJson(Map<String, dynamic> json) {
    return CommunityCommentCommand(
      body: (() {
        final value = json['body']?.toString();
        if (value == null) {
          throw FormatException('CommunityCommentCommand.body is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'body': body,
    };
  }
}

class CommunityReactionCommand {
  final String reactionType;
  final bool active;

  CommunityReactionCommand({
    required this.reactionType,
    required this.active
  });

  factory CommunityReactionCommand.fromJson(Map<String, dynamic> json) {
    return CommunityReactionCommand(
      reactionType: (() {
        final value = json['reactionType']?.toString();
        if (value == null) {
          throw FormatException('CommunityReactionCommand.reactionType is required');
        }
        return value;
      })(),
      active: (() {
        final value = json['active'];
        if (value is! bool) {
          throw FormatException('CommunityReactionCommand.active is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'reactionType': reactionType,
      'active': active,
    };
  }
}

class CommunityReactionSetAccepted {
  final bool accepted;
  final String? resourceId;
  final String? status;
  final int reactionCount;

  CommunityReactionSetAccepted({
    required this.accepted,
    this.resourceId,
    this.status,
    required this.reactionCount
  });

  factory CommunityReactionSetAccepted.fromJson(Map<String, dynamic> json) {
    return CommunityReactionSetAccepted(
      accepted: (() {
        final value = json['accepted'];
        if (value is! bool) {
          throw FormatException('CommunityReactionSetAccepted.accepted is required');
        }
        return value;
      })(),
      resourceId: json['resourceId']?.toString(),
      status: json['status']?.toString(),
      reactionCount: (() {
        final value = json['reactionCount'];
        if (value is! int) {
          throw FormatException('CommunityReactionSetAccepted.reactionCount is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'accepted': accepted,
      'resourceId': resourceId,
      'status': status,
      'reactionCount': reactionCount,
    };
  }
}

class CommunityModerationCommand {
  final String reviewState;
  final String? reason;

  CommunityModerationCommand({
    required this.reviewState,
    this.reason
  });

  factory CommunityModerationCommand.fromJson(Map<String, dynamic> json) {
    return CommunityModerationCommand(
      reviewState: (() {
        final value = json['reviewState']?.toString();
        if (value == null) {
          throw FormatException('CommunityModerationCommand.reviewState is required');
        }
        return value;
      })(),
      reason: json['reason']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'reviewState': reviewState,
      'reason': reason,
    };
  }
}

class CommunityFeatureCommand {
  final bool? featured;

  CommunityFeatureCommand({
    this.featured
  });

  factory CommunityFeatureCommand.fromJson(Map<String, dynamic> json) {
    return CommunityFeatureCommand(
      featured: json['featured'] is bool ? json['featured'] : null
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'featured': featured,
    };
  }
}

class CommunityPinCommand {
  final bool? pinned;

  CommunityPinCommand({
    this.pinned
  });

  factory CommunityPinCommand.fromJson(Map<String, dynamic> json) {
    return CommunityPinCommand(
      pinned: json['pinned'] is bool ? json['pinned'] : null
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'pinned': pinned,
    };
  }
}

class CommunityPublicationReadiness {
  final bool ready;
  final bool degraded;
  final List<String> issues;

  CommunityPublicationReadiness({
    required this.ready,
    required this.degraded,
    required this.issues
  });

  factory CommunityPublicationReadiness.fromJson(Map<String, dynamic> json) {
    return CommunityPublicationReadiness(
      ready: (() {
        final value = json['ready'];
        if (value is! bool) {
          throw FormatException('CommunityPublicationReadiness.ready is required');
        }
        return value;
      })(),
      degraded: (() {
        final value = json['degraded'];
        if (value is! bool) {
          throw FormatException('CommunityPublicationReadiness.degraded is required');
        }
        return value;
      })(),
      issues: (() {
        final list = _sdkworkAsList(json['issues']);
        if (list == null) {
          throw FormatException('CommunityPublicationReadiness.issues is required');
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'ready': ready,
      'degraded': degraded,
      'issues': issues.map((item) => item).toList(),
    };
  }
}

class MembersRetrieveResponse {
  final int code;
  final dynamic data;
  final String traceId;

  MembersRetrieveResponse({
    required this.code,
    required this.data,
    required this.traceId
  });

  factory MembersRetrieveResponse.fromJson(Map<String, dynamic> json) {
    return MembersRetrieveResponse(
      code: (() {
        final value = json['code'];
        if (value is! int) {
          throw FormatException('MembersRetrieveResponse.code is required');
        }
        return value;
      })(),
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        if (map == null) {
          throw FormatException('MembersRetrieveResponse.data is required');
        }
        return map;
      })(),
      traceId: (() {
        final value = json['traceId']?.toString();
        if (value == null) {
          throw FormatException('MembersRetrieveResponse.traceId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}
