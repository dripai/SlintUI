# Avatar

状态：已实现（P2）

## 用途与边界
展示用户、团队或实体的紧凑身份图形，不承担图片上传。

## 公开 API
`source`、`initials`、`accessible-name`、`size`、前景/背景语义色、`circular`。

## 状态与交互
非交互组件；图片存在时覆盖文字回退，支持三种 ControlSize。

## 无障碍与本地化
使用 image 角色和显式名称；缩写由宿主依据姓名与 Locale 生成。

## Gallery、测试与限制
Gallery 展示文字回退；编译和截图验证尺寸。Slint 图片类型不公开加载失败事件，网络加载与失败回退由宿主更新 `source`/`initials`。

遵循四份全局规范。
