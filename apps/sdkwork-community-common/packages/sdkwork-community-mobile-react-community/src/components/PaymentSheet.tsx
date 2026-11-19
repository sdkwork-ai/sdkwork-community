import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { cn, IconButton } from "@sdkwork/ui-mobile-react";
import { MessageSquare, Check, X, Lock } from "lucide-react";
import { getCommunityCurrentUser } from "../services/communityAuthSessionPort";
import { useNavigate, useLocation } from "react-router";
import type { MembershipTier } from "../types";
import { CommunityImage } from "./CommunityImage";

/** Purchase confirmation payload: which tier, which package (yearly vs
 * lifetime) and which payment method. */
export interface CirclePurchaseConfirm {
  tier: MembershipTier;
  /** order packageId: the yearly (catalogPackageId) or lifetime package. */
  packageId: string;
  paymentMethod: string;
  /** true when the purchase is the lifetime package. */
  isLifetime: boolean;
}

interface PaymentSheetProps {
  communityName: string;
  communityCoverImage: string;
  tiers: MembershipTier[];
  onClose: () => void;
  onConfirm: (confirm: CirclePurchaseConfirm) => void;
}

export const PaymentSheet: React.FC<PaymentSheetProps> = ({
  communityName,
  communityCoverImage,
  tiers,
  onClose,
  onConfirm,
}) => {
  const { t } = useTranslation();
  const [selectedTierId, setSelectedTierId] = useState<string | null>(null);
  // Lifetime purchase selected for the current tier (defaults to yearly).
  const [isLifetime, setIsLifetime] = useState(false);
  // Payment method values follow the sdkwork-order contract (wechat_pay/alipay).
  const [selectedPayment, setSelectedPayment] = useState<'wechat_pay'|'alipay'|null>(null);
  const [isWeChat, setIsWeChat] = useState(false);
  const [isAlipay, setIsAlipay] = useState(false);

  const navigate = useNavigate();
  const location = useLocation();

  // Callers open the sheet before tiers resolve, so preselect the first tier
  // once the list arrives. Also recover a stale selection (e.g. the tier list
  // was refreshed and the previously chosen tier no longer exists) instead of
  // leaving the sheet in a dead "select a tier" state.
  useEffect(() => {
    if (tiers.length === 0) {
      if (selectedTierId !== null) {
        setSelectedTierId(null);
      }
      return;
    }
    if (selectedTierId && tiers.some((tier) => tier.id === selectedTierId)) {
      return;
    }
    setSelectedTierId(tiers[0].id);
  }, [tiers, selectedTierId]);

  useEffect(() => {
    const user = getCommunityCurrentUser();
    if (!user) {
      const currentPath = encodeURIComponent(location.pathname + location.search);
      navigate(`/login?redirect=${currentPath}`);
    }
  }, [navigate, location]);

  useEffect(() => {
    // Pick the default method the embedded browser can actually settle.
    const ua = navigator.userAgent.toLowerCase();
    const isWx = ua.includes('micromessenger');
    const isAli = ua.includes('alipayclient');

    setIsWeChat(isWx);
    setIsAlipay(isAli);
  }, []);

  const selectedTier = tiers.find((tier) => tier.id === selectedTierId) ?? null;

  // Reset the term when switching tiers (lifetime may not be offered).
  useEffect(() => {
    setIsLifetime(false);
  }, [selectedTierId]);

  const handleConfirm = () => {
    if (!selectedTier) return;
    const paymentMethod = selectedPayment ?? (isAlipay ? 'alipay' : 'wechat_pay');
    const packageId = isLifetime && selectedTier.lifetimePackageId
      ? selectedTier.lifetimePackageId
      : selectedTier.catalogPackageId;
    if (!packageId) return;
    onConfirm({
      tier: selectedTier,
      packageId,
      paymentMethod,
      isLifetime: isLifetime && Boolean(selectedTier.lifetimePackageId),
    });
  };

  return (
    <div className="fixed inset-0 z-50 flex items-end justify-center bg-black/50" onClick={onClose}>
      <div
        className="w-full max-w-md bg-white dark:bg-[#1C1C1E] rounded-t-2xl p-4 pb-8 max-h-[85vh] overflow-y-auto"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center justify-between mb-4">
          <h3 className="text-[17px] font-bold text-text-main">{t('community.auto_open_membership', '开通圈子会员')}</h3>
          <IconButton icon={<X className="w-5 h-5" />} onClick={onClose} aria-label="close" />
        </div>

        {/* 圈子信息 */}
        <div className="flex items-center gap-3 mb-4">
          <CommunityImage src={communityCoverImage} alt="" fallbackSeed={communityName} className="w-12 h-12 rounded-xl object-cover" />
          <div className="flex flex-col">
            <span className="text-[15px] font-semibold text-text-main">{communityName}</span>
            <span className="text-[12px] text-text-sub">{t('community.auto_25f42a69', '付费圈子')}</span>
          </div>
        </div>

        {/* 会员等级选择 */}
        {tiers.length === 0 ? (
          <div className="flex flex-col items-center gap-3 py-8">
            <div className="w-12 h-12 bg-blue-500/10 rounded-full flex items-center justify-center">
              <Lock className="w-6 h-6 text-blue-500" />
            </div>
            <p className="text-[14px] text-text-sub text-center leading-relaxed px-4">
              {t('community.auto_no_purchasable_tiers', '该圈子暂无可购买的会员套餐，请稍后再试或联系圈主')}
            </p>
            <button
              onClick={onClose}
              className="px-6 py-2 rounded-full border border-black/10 dark:border-white/10 text-[14px] font-medium text-text-main"
            >
              {t('community.auto_got_it', '知道了')}
            </button>
          </div>
        ) : (
        <>
        <div className="flex flex-col gap-2 mb-4">
          {tiers.map((tier) => (
            <button
              key={tier.id}
              onClick={() => setSelectedTierId(tier.id)}
              className={cn(
                "flex items-center justify-between w-full rounded-xl border px-3 py-3 text-left transition-colors",
                selectedTierId === tier.id
                  ? "border-blue-500 bg-blue-50 dark:bg-blue-500/10"
                  : "border-black/10 dark:border-white/10",
              )}
            >
              <div className="flex flex-col gap-0.5">
                <span className="text-[15px] font-semibold text-text-main">{tier.name}</span>
                {tier.description && (
                  <span className="text-[12px] text-text-sub line-clamp-1">{tier.description}</span>
                )}
                <span className="text-[12px] text-text-sub">
                  {tier.durationDays >= 365
                    ? t('community.auto_duration_years', '{{years}} 年有效', { years: Math.round(tier.durationDays / 365) })
                    : t('community.auto_duration_days', '{{days}} 天有效', { days: tier.durationDays })}
                </span>
              </div>
              <div className="flex flex-col items-end gap-0.5">
                <span className="text-[16px] font-bold text-red-500">¥{tier.price}<span className="text-[11px] font-normal text-text-sub">/年</span></span>
                {tier.lifetimePrice ? (
                  <span className="text-[11px] text-text-sub">终身 ¥{tier.lifetimePrice}</span>
                ) : null}
                <span
                  className={cn(
                    "w-4 h-4 rounded-full border flex items-center justify-center",
                    selectedTierId === tier.id ? "border-blue-500" : "border-black/20 dark:border-white/25",
                  )}
                >
                  {selectedTierId === tier.id && <Check className="w-3 h-3 text-blue-500" />}
                </span>
              </div>
            </button>
          ))}
        </div>

        {/* 权益预览 */}
        {selectedTier && selectedTier.benefits.length > 0 && (
          <div className="mb-4 rounded-xl bg-chat-active-bg p-3 flex flex-col gap-1.5">
            {selectedTier.benefits.map((benefit) => (
              <span key={benefit} className="text-[13px] text-text-main flex items-center gap-1.5">
                <Check className="w-3.5 h-3.5 text-blue-500" /> {benefit}
              </span>
            ))}
          </div>
        )}

        {/* 支付周期：终身/按年（档位提供终身价时） */}
        {selectedTier && selectedTier.lifetimePrice ? (
          <div className="flex flex-col gap-2 mb-4">
            <span className="text-[13px] text-text-sub">{t('community.auto_purchase_term', '购买周期')}</span>
            <div className="flex gap-2">
              <button
                onClick={() => setIsLifetime(false)}
                className={cn(
                  "flex-1 flex flex-col items-center gap-0.5 rounded-xl border py-2.5 text-[14px] font-medium",
                  !isLifetime ? "border-blue-500 bg-blue-50 dark:bg-blue-500/10 text-blue-500" : "border-black/10 dark:border-white/10 text-text-main",
                )}
              >
                <span>{t('community.auto_term_yearly', '按年')}</span>
                <span className="text-[12px] font-bold">¥{selectedTier.price}</span>
              </button>
              <button
                onClick={() => setIsLifetime(true)}
                className={cn(
                  "flex-1 flex flex-col items-center gap-0.5 rounded-xl border py-2.5 text-[14px] font-medium",
                  isLifetime ? "border-blue-500 bg-blue-50 dark:bg-blue-500/10 text-blue-500" : "border-black/10 dark:border-white/10 text-text-main",
                )}
              >
                <span>{t('community.auto_term_lifetime', '终身')}</span>
                <span className="text-[12px] font-bold">¥{selectedTier.lifetimePrice}</span>
              </button>
            </div>
          </div>
        ) : null}

        {/* 支付方式 */}
        {!isWeChat && !isAlipay && (
          <div className="flex flex-col gap-2 mb-4">
            <span className="text-[13px] text-text-sub">{t('community.auto_2b2c2b90', '支付方式')}</span>
            <div className="flex gap-2">
              <button
                onClick={() => setSelectedPayment('wechat_pay')}
                className={cn(
                  "flex-1 flex items-center justify-center gap-2 rounded-xl border py-2.5 text-[14px] font-medium",
                  selectedPayment === 'wechat_pay' ? "border-[#07C160] bg-[#07C160]/10 text-[#07C160]" : "border-black/10 dark:border-white/10 text-text-main",
                )}
              >
                <MessageSquare className="w-4 h-4" /> {t('community.auto_2cb6c4bc', '微信支付')}
              </button>
              <button
                onClick={() => setSelectedPayment('alipay')}
                className={cn(
                  "flex-1 flex items-center justify-center gap-2 rounded-xl border py-2.5 text-[14px] font-medium",
                  selectedPayment === 'alipay' ? "border-[#1677FF] bg-[#1677FF]/10 text-[#1677FF]" : "border-black/10 dark:border-white/10 text-text-main",
                )}
              >
                {t('community.auto_185bd34', '支付宝')}
              </button>
            </div>
          </div>
        )}

        {/* 立即支付 */}
        <button
          onClick={handleConfirm}
          disabled={!selectedTier}
          className="w-full rounded-xl bg-blue-500 py-3 text-[15px] font-semibold text-white disabled:opacity-40"
        >
          {selectedTier
            ? t('community.auto_pay_now', '立即支付 ¥{{price}}', { price: isLifetime && selectedTier.lifetimePrice ? selectedTier.lifetimePrice : selectedTier.price })
            : t('community.auto_select_tier', '请选择会员等级')}
        </button>

        <div className="mt-3 flex items-center justify-center gap-1 text-[12px] text-text-sub">
          <Lock className="w-3 h-3" />
          {t('community.auto_secure_cashier', '支付将通过安全收银台完成')}
        </div>
        </>
        )}
      </div>
    </div>
  );
};
