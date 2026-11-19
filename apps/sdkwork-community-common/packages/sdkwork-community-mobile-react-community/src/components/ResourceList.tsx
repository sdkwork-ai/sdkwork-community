import { useTranslation } from "react-i18next";
import React from "react";
import { Resource } from "../types";
import { IconButton, showToast } from "@sdkwork/ui-mobile-react";
import { FileText, Download } from "lucide-react";
import {
  getCommunityMediaRuntime,
  isCommunityMediaRuntimeConfigured,
} from "../services/communityMediaRuntimePort";

interface ResourceListProps {
  resources: Resource[];
}

export const ResourceList: React.FC<ResourceListProps> = ({ resources }) => {
  const { t } = useTranslation();

  const openResource = async (resource: Resource) => {
    let url = resource.url;
    if (url.startsWith("drive://")) {
      if (!isCommunityMediaRuntimeConfigured()) {
        showToast(t("community.auto_resource_unavailable", "资源地址不可用"));
        return;
      }
      const resolved = await getCommunityMediaRuntime().resolveDisplayUrl?.(url);
      if (!resolved) {
        showToast(t("community.auto_resource_unavailable", "资源地址不可用"));
        return;
      }
      url = resolved;
    }
    if (!url || url === "#") {
      showToast(t("community.auto_resource_unavailable", "资源地址不可用"));
      return;
    }
    window.open(url, "_blank", "noopener,noreferrer");
  };

  return (
    <div className="pb-24 flex flex-col bg-white dark:bg-[#1C1C1E]">
      {resources.map(res => (
        <div key={res.id} className="bg-white dark:bg-[#1C1C1E] p-4 flex gap-3 items-center border-b border-black/5 dark:border-white/5 active:bg-black/5 dark:active:bg-white/5 transition-colors">
          <div className="w-12 h-12 rounded-xl bg-orange-500/10 flex items-center justify-center shrink-0">
            <FileText className="w-6 h-6 text-orange-500" />
          </div>
          <div className="flex-1 overflow-hidden">
            <div className="text-[15px] font-bold text-text-main truncate mb-1">
              {res.title}
              {res.reviewState && res.reviewState !== "approved" ? (
                <span className="ml-2 rounded-full bg-amber-500/10 px-2 py-0.5 text-[11px] font-medium text-amber-600">
                  {t('community.auto_pending_review', '审核中')}
                </span>
              ) : null}
            </div>
            <div className="flex items-center gap-2 text-[12px] text-text-sub">
              <span className="uppercase px-1 border border-black/10 dark:border-white/15 rounded">{res.type}</span>
              {res.size && <span>{res.size}</span>}
              <span>{t('community.auto_4e0f68b', '· {{uploadedBy}} 分享', { uploadedBy: res.uploadedBy })}</span>
            </div>
          </div>
          <IconButton
            icon={<Download className="w-5 h-5 text-blue-500" />}
            className="bg-blue-500/10 shrink-0 w-10 h-10"
            onClick={() => void openResource(res)}
          />
        </div>
      ))}
      {resources.length === 0 && (
        <div className="h-40 flex items-center justify-center text-text-sub">{t('community.auto_3028e9ea', '暂无资源')}</div>
      )}
    </div>
  );
};
