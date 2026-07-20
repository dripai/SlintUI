# ComboBox

状态：已实现（P1）。源码：`ui/controls/combo-box.slint`。

## 用途与边界
用于可编辑文本与可选候选列表并存；必须从固定项选择时使用 Select，纯建议场景使用 AutoComplete。

## 公开 API
复用 AutoComplete 的 `text`、`suggestions`、`current-index`、`enabled`、`validation`、`show()/close()/choose()`、`edited/selected/accepted`。

## 状态与交互
覆盖输入、候选打开、Focused、Selected、Disabled、Error、自由文本和禁用候选；选择候选或提交自由文本是不同回调。

## 无障碍与本地化
使用 combobox 语义并暴露 expanded；候选名称和详情由宿主本地化，输入值不自动改写。

## Gallery、测试与限制
Gallery 展示自由输入和候选；Harness 覆盖正常、禁用候选和边界。当前与 AutoComplete 共用视觉实现，但公开语义强调自由值可提交。

遵循四份全局规范。
