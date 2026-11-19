import { useTranslation } from "react-i18next";
import React, { useState, useRef, useEffect } from "react";
import { useParams, useNavigate, useSearchParams } from "react-router";
import { CommunityService } from "../services/CommunityService";
import { isCommunityMediaRuntimeConfigured } from "../services/communityMediaRuntimePort";
import { showToast, cn } from "@sdkwork/ui-mobile-react";
import { X, Plus } from "lucide-react";
import { useEditor, EditorContent } from '@tiptap/react';
import StarterKit from '@tiptap/starter-kit';
import Placeholder from '@tiptap/extension-placeholder';

export const CommunityPostCreate: React.FC = () => {
  const { t } = useTranslation();
const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const tab = (searchParams.get("tab") ?? "feeds").trim();
  const isResource = tab === "resources";
  const isCustomTab = ["news", "docs", "repos", "software"].includes(tab);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [title, setTitle] = useState("");
  const [resourceUrl, setResourceUrl] = useState("");
  const [resourceType, setResourceType] = useState("link");
  // Upload-ready File objects; data-URL previews are derived for display.
  const [images, setImages] = useState<File[]>([]);
  const [imagePreviews, setImagePreviews] = useState<string[]>([]);
  const [mediaUploadEnabled, setMediaUploadEnabled] = useState(true);
  const fileInputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    // Hosts without a drive-backed media runtime (e.g. standalone community
    // H5) cannot upload images; hide the picker instead of faking media.
    setMediaUploadEnabled(isCommunityMediaRuntimeConfigured());
  }, []);

  const editor = useEditor({
    extensions: [
      StarterKit,
      Placeholder.configure({
        placeholder: t('community.auto_thought_placeholder', '这一刻的想法...'),
        emptyEditorClass: 'is-editor-empty',
      }),
    ],
    content: '',
    editorProps: {
      attributes: {
        class: 'prose prose-sm max-w-none focus:outline-none min-h-[120px] text-[16px] text-black dark:text-white p-4 bg-transparent',
      },
    },
  });

  const handleImagePick = () => {
  if (images.length >= 9) {
      showToast(t('community.auto_fn_n2d485cc', '最多只能选择9张照片'));
      return;
    }
    fileInputRef.current?.click();
  };

  const handleFileChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const files = Array.from(e.target.files || []) as File[];
    if (!files.length) return;

    if (fileInputRef.current) {
        fileInputRef.current.value = '';
    }

    const availableSlots = 9 - images.length;
    const filesProcess = files.slice(0, availableSlots);

    if (files.length > availableSlots) {
       showToast(t('community.auto_max_9_photos', '最多只能选择9张图片，已截取前{{count}}张', { count: availableSlots }));
    }

    try {
      // Keep the File objects for upload; read data-URLs only for preview.
      const previewPromises = filesProcess.map(file => {
        return new Promise<string>((resolve, reject) => {
          const reader = new FileReader();
          reader.onload = (event) => {
            if (event.target?.result) {
              resolve(event.target.result as string);
            } else {
              reject(new Error("Failed to read file"));
            }
          };
          reader.readAsDataURL(file);
        });
      });

      const newPreviews = await Promise.all(previewPromises);
      setImages(prev => [...prev, ...filesProcess]);
      setImagePreviews(prev => [...prev, ...newPreviews]);
    } catch (err) {
       console.error("Image loading failed:", err);
       showToast(t('community.auto_fn_e0f746b', '图片读取失败，请重试'));
    }
  };

  const removeImage = (indexToRemove: number) => {
  setImages(prev => prev.filter((_, idx) => idx !== indexToRemove));
  setImagePreviews(prev => prev.filter((_, idx) => idx !== indexToRemove));
  };

  const handleSubmit = async () => {
    if (!id) return;
    const content = editor?.getText() || "";
    if (isResource) {
      if (!title.trim() || !resourceUrl.trim()) {
        showToast(t('community.auto_resource_required', '请填写资源标题和地址'));
        return;
      }
      setIsSubmitting(true);
      try {
        const created = await CommunityService.createResource(id, {
          title,
          url: resourceUrl,
          type: resourceType,
          description: content,
        });
        showToast(created.reviewState === "approved"
          ? t('community.auto_fn_28260f86', '发表成功')
          : t('community.auto_pending_submitted', '已提交审核，审核通过后展示'));
        navigate(-1);
      } catch {
        showToast(t('community.auto_fn_b5c6cde', '发表失败，请重试'));
      } finally {
        setIsSubmitting(false);
      }
      return;
    }
    if (!content.trim() && images.length === 0) {
      showToast(t('community.auto_fn_n707125e8', '请输入内容或上传图片'));
      return;
    }
    setIsSubmitting(true);
    try {
      const created = await CommunityService.createPost(id, content, images, {
        ...(title.trim() ? { title } : {}),
        ...(isCustomTab ? { tags: [tab] } : {}),
      });
      showToast(created.reviewState === "approved"
        ? t('community.auto_fn_28260f86', '发表成功')
        : t('community.auto_pending_submitted', '已提交审核，审核通过后展示'));
      // We just go back. For actual replace we would need the previous path, 
      // but going back is usually sufficient for create pages.
      navigate(-1);
    } catch {
      showToast(t('community.auto_fn_b5c6cde', '发表失败，请重试'));
    } finally {
      setIsSubmitting(false);
    }
  };

  const isEmpty = editor?.isEmpty;
  const isPublishDisabled = isSubmitting || (isResource
    ? !title.trim() || !resourceUrl.trim()
    : isEmpty && images.length === 0);

  return (
    <div className="flex flex-col h-full bg-white dark:bg-[#111111] overflow-hidden relative selection:bg-blue-500/30">
      <style>{`
        .ProseMirror p.is-editor-empty:first-child::before {
          color: #A3A3A3;
          content: attr(data-placeholder);
          float: left;
          height: 0;
          pointer-events: none;
        }
        .dark .ProseMirror p.is-editor-empty:first-child::before {
          color: #52525B;
        }
        .ProseMirror {
          outline: none !important;
          word-break: break-word;
        }
      `}</style>

      {/* Header */}
      <header className="h-[56px] px-4 flex items-center justify-between sticky top-0 z-10 shrink-0 pt-safe bg-white dark:bg-[#111111]">
        <button 
          onClick={() => navigate(-1)}
          className="text-[16px] text-black dark:text-white active:opacity-50 p-2 -ml-2"
        >{t('community.auto_a9472', '取消')}</button>
        
        <button 
          className={cn(
             "px-4 py-1.5 rounded-[4px] text-[15px] font-medium transition-all flex items-center",
             isPublishDisabled 
                ? "bg-[#E5E5EA] dark:bg-[#2C2C2E] text-black/30 dark:text-white/30" 
                : "bg-[#07C160] text-white active:bg-[#06ad56]"
          )}
          onClick={handleSubmit}
          disabled={isPublishDisabled}
        >{isSubmitting ? t('community.auto_publishing', '发表中...') : t('community.auto_publish', '发表')}</button>
      </header>

      <div className="flex-1 overflow-y-auto w-full flex flex-col mb-safe">
         {(isResource || isCustomTab) && (
           <div className="px-4 pt-3 flex flex-col gap-2">
             <input
               value={title}
               onChange={(event) => setTitle(event.target.value)}
               placeholder={t('community.auto_entry_title', '标题')}
               className="w-full bg-[#F2F2F7] dark:bg-[#2C2C2E] px-4 py-3 rounded-xl outline-none text-[15px] text-text-main"
             />
             {isResource && (
               <>
                 <input
                   value={resourceUrl}
                   onChange={(event) => setResourceUrl(event.target.value)}
                   placeholder={t('community.auto_resource_url', '资源地址（链接或文件 URL）')}
                   className="w-full bg-[#F2F2F7] dark:bg-[#2C2C2E] px-4 py-3 rounded-xl outline-none text-[15px] text-text-main"
                 />
                 <select
                   value={resourceType}
                   onChange={(event) => setResourceType(event.target.value)}
                   className="w-full bg-[#F2F2F7] dark:bg-[#2C2C2E] px-4 py-3 rounded-xl outline-none text-[15px] text-text-main"
                 >
                   <option value="link">link</option>
                   <option value="doc">doc</option>
                   <option value="pdf">pdf</option>
                   <option value="video">video</option>
                   <option value="repo">repo</option>
                   <option value="software">software</option>
                 </select>
               </>
             )}
             {isCustomTab && (
               <p className="px-1 text-[12px] text-text-sub">
                 {t('community.auto_tab_tag_hint', '发布后会显示在“{{tab}}”标签页', { tab })}
               </p>
             )}
           </div>
         )}
         <EditorContent editor={editor} className="w-full flex-shrink-0" />
         
         {/* Internal Image Grid (posts only; resources are addressed by URL) */}
         <div className={isResource ? "hidden" : "px-4 pb-4"}>
            <div className="grid grid-cols-3 gap-2">
                {imagePreviews.map((img, index) => (
                    <div key={index} className="relative aspect-square rounded-md overflow-hidden bg-[#F2F2F7] dark:bg-[#2C2C2E]">
                        <img src={img} alt="" className="w-full h-full object-cover" />
                        <button 
                            className="absolute top-1 right-1 w-5 h-5 bg-black/50 rounded-full flex items-center justify-center pointer-events-auto active:scale-95 transition-transform"
                            onClick={(e) => { e.stopPropagation(); removeImage(index); }}
                        >
                            <X className="w-3.5 h-3.5 text-white" />
                        </button>
                    </div>
                ))}
                
                {mediaUploadEnabled && images.length < 9 && (
                    <div 
                        className="aspect-square rounded-md bg-[#F2F2F7] dark:bg-[#2C2C2E] flex flex-col items-center justify-center cursor-pointer active:bg-[#E5E5EA] dark:active:bg-[#3A3A3C] transition-colors"
                        onClick={handleImagePick}
                    >
                        <Plus className="w-8 h-8 text-[#8E8E93] dark:text-[#636366]" strokeWidth={1.5} />
                    </div>
                )}
            </div>
         </div>
      </div>
      
      {/* Hidden File Input */}
      <input 
         type="file" 
         multiple
         accept="image/*"
         className="hidden"
         ref={fileInputRef}
         onChange={handleFileChange}
      />
    </div>
  );
};
