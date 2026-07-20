# ImageView

状态：已实现（P2）

## 用途与边界
显示单张图片及加载、空、失败状态，不提供图片编辑。

## 公开 API
`source`、`state: ImageViewState`、`fit: ImageFit`、状态文案和无障碍名称；`retry-requested()`。

## 状态与交互
覆盖 loading、ready、empty、error；失败态整个区域可请求重试。

## 无障碍与本地化
使用 image 角色，替代说明和状态文案由调用方提供；状态使用 SVG 图标和文本共同表达。

## Gallery、测试与限制
Gallery 展示空状态；编译和截图验收。Slint 不向该包装层公开通用加载错误事件，宿主必须控制 `state`，且组件不含缩放、平移和缓存。

遵循四份全局规范。
