# Theme

状态：已实现。源码：`ui/tokens/theme.slint`。

## 用途与边界

为每个顶层窗口提供主题、密度、Locale、Direction、文本/预览缩放、减少动效和语义 Token。它不负责读取操作系统设置；系统高对比度、减少动效和多窗口同步由宿主或平台增强层完成。

## 公开 API

- 可写环境：`mode: ThemeMode = light`、`density: Density = regular`、`direction: Direction = ltr`、`locale: string = "zh-CN"`、`text-scale: float = 1.0`、`preview-scale: float = 1.0`、`reduced-motion: bool = false`。
- 只读 Token：背景、文本、图标、边框、功能色、排版、间距、圆角、尺寸与动效属性；业务代码不得覆盖其派生关系。

## 状态、交互与无障碍

Theme 不接收指针或键盘输入，也没有焦点和可访问节点。环境切换必须由宿主原子更新；组件通过绑定自动响应。

## 本地化、Gallery 与测试

Gallery 顶栏覆盖三种主题、三种密度、100%–200% 缩放、中文/英文/阿拉伯语和 LTR/RTL。编译与截图验证 Token 在这些环境下可用；操作系统自动探测和多平台真机结果单独记录。

## 原生能力与限制

Slint global 不跨窗口共享，宿主必须逐窗口同步。完整约束遵循 [`../foundations.md`](../foundations.md)、[`../accessibility.md`](../accessibility.md) 和 [`../content-and-localization.md`](../content-and-localization.md)。
