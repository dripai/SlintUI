# PopupMenu / PopupMenuItem

状态：已实现（P1）

## 用途与边界
承载离散命令、选中标记、快捷键和分组；复杂交互表单使用 Popover，不用于必须确认的操作。

## 公开 API
`entries: [PopupMenuEntry]`、`current-index`、`popup-width/height`、`open`；`selected`、`submenu-requested`、`closed`；`show()`、`close()`、`activate(index)`。PopupMenuItem 公开 `entry`、`triggered` 和 `submenu-requested`。

## 状态与交互
使用原生 PopupWindow 的外部点击关闭；支持 Hover、Highlighted、Checked、Disabled、Separator，方向键、Home/End、Enter/Space 和 Esc。

## 无障碍与本地化
菜单采用 list/list-item 公开角色，暴露名称、checked、expanded 和 enabled。文案、快捷键显示由宿主本地化；方向图标按后续子菜单所有权处理。

## Gallery、测试与限制
Gallery 展示分组、快捷键、checked、disabled 和 submenu；Harness 验证禁用项不触发。多级子菜单由宿主响应 `submenu-requested` 建立，Slint 1.17.1 不提供完整原生 Menu API。

遵循四份全局规范。
