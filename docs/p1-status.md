# P1 里程碑 D 实现状态

本文记录里程碑 D 已落地的核心桌面组件。它不表示 `component-inventory.md` 中所有 P1 条目均已完成，也不包含优先级为“平台”的系统 API。

## 已实现范围

- 浮层基础：Overlay。
- 导航与菜单：Tabs、PopupMenu、PopupMenuItem、ContextMenu。
- 数据与布局：Table、Tree、SplitPane、Form。
- 反馈：Toast、ModalDialog、ConfirmDialog。

## 验证范围

- Gallery 第 6 页覆盖浅色常规密度的代表性桌面组合，并继续支持主题、密度、Locale、RTL 和 100%–200% 预览缩放切换。
- 自动交互测试覆盖 Tabs 的同值/禁用保护、Tree 选择与展开、SplitPane 比例变化、PopupMenu 禁用项、Form 无效提交。
- 固定截图为 `tests/screenshots/gallery-light-desktop-p1-100.png`，尺寸和 SHA-256 进入 manifest。

## Slint 1.17.1 限制

- PopupWindow 提供原生定位、外部点击关闭和焦点窗口；多级子菜单仍由宿主响应 `submenu-requested` 后显式建立所有权链。
- StandardTableView 提供排序、选择、键盘和滚动；当前 Table 不承诺虚拟化、编辑、固定列或自定义单元格。
- 稳定组件语法没有两个公开命名内容插槽；SplitPane 公开两侧几何属性，调用方将两个子区域绑定到这些几何值。
- Tree 使用宿主提供的扁平可见模型；懒加载和父子可见性由数据层更新，组件不猜测层级数据。
- ModalDialog 是独立顶层窗口。宿主负责创建、所有者生命周期和多窗口主题同步；核心 crate 不模拟平台原生模态窗口。
- Form 不反射任意子组件，错误聚合通过显式 `FormError` 模型，焦点定位通过 `focus-error-requested` 交给宿主关联字段。

## 仍待实现

其余 P1 组件、Windows/macOS/Linux 平台增强、三平台真机屏幕阅读器和真实 DPI 验证继续保持“待实现”。
