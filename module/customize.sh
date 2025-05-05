# 注意 这不是占位符！！这个代码的作用是将模块里的东西全部塞系统里，然后挂上默认权限
SKIPUNZIP=0

ui_print "欢迎使用life-death-scheduler"
ui_print "此调度适用于大部分设备"
ui_print "有问题请到687235389反馈"

sleep 0.3

ui_print "正在设置权限"

set_perm_recursive $MODPATH 0 0 0755 0644
set_perm $MODPATH/life-death-scheduler 0 0 0755