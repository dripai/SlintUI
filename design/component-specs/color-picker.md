# ColorPicker

状态：已实现（P2）

## 用途与边界
从预定义语义色板选择颜色；复杂设计工具使用专用编辑器。

## 公开 API
`value: color`、`options: [ColorOption]`、`value-label`、状态和名称；`changed(index, value)`；`show`、`close`、`select(index)`。

## 状态与交互
覆盖色块、打开、Hover、Disabled 和选项禁用；宿主受控更新格式标签。

## 无障碍与本地化
根为 combobox，当前值以 `value-label` 朗读，色彩不作为唯一信息来源。

## Gallery、测试与限制
Gallery 与 Harness 覆盖预设色选择。当前不内建自由取色、透明度滑杆、HEX/RGB 文本解析和吸管。

遵循四份全局规范。
