# Button

状态：已实现。源码：`ui/controls/button.slint`。

- 用途：离散操作与可选的可切换操作。
- API：`text`、`icon`、`variant`、`size`、`enabled`、`loading`、`checkable`、`checked`、`accessible-name`、`clicked()`、`activate()`。
- 行为：支持 default/primary/danger/text/link，三种尺寸，Hover、Pressed、Focused、Disabled、Checked、Loading；Enter/Space 激活，Loading 阻止重复触发。
- 原生复用：使用公开 `TouchArea`、`FocusScope` 和可访问回调；标准 `Button` 不开放完整 Token 外观，因此未直接继承。

