# wfc

An implementation of the Wave Function Collapse algorithm in 2D.

## Example

The following exmaple was generated while using the `MazeTile` tile that can be found inside [tile.rs](src/tile.rs).

```
╋┫ ┏┓┃┃┣╋━┛┃ ┗┓┗┫┣━┛┗┛┣┫ ┃  ┃┣━┓┣┫┗┫┃┗╋┛ ┃  ┃┃┏┻┛┏┻┫ ┣┫┏╋┓┃┃┗┓ ┃┣┻┓┏━┓┏┻┓ ┣┫┣┻━╋
┣╋━┛┃┗┛┣┫  ┃  ┣━┫┗┓┏┓ ┃┃ ┣┓ ┃┣━┛┗┛ ┣╋┓┣┳┓┣┓┏┫┗┻┓ ┣━┻━┫┃┗┻╋┛┃ ┃ ┣┫┏┫┃ ┃┗┓┗━┻┫┃  ┃
┛┃┏┳┫  ┃┣┓┏┫┏┳╋┓┣┳┻┻┛ ┣╋━┫┗┓┗╋━┓┏━┳╋┛┗╋╋┛┗╋╋╋━━┛┏┫┏┳━┫┃┏┓┣━┻┓┣┓┃┗┫┃┣━┻┓┗━━┳╋┫  ┣
━┛┗┫┣┓┏┫┣┻┻┛┗┛┗┫┃┃    ┃┃ ┣┓┣━╋━┛┗┳┛┃  ┣┫┏┳┻┫┗━┓┏┻┻┛┣┓┃┃┗╋┛  ┃┗┻╋┓┣┻┛  ┣┳━┓┣┻┛  ┃
  ┏┻┫┗┫┗┻━┓    ┣┫┣┳┳┳┳┛┃ ┣┻┛ ┃┏━┳┻━┫  ┣╋┫┃ ┃  ┣┫┏┳┳┻╋┛┃ ┃   ┃  ┣╋┛    ┣╋┓┣┻┳━┳┳┛
  ┣┓┃ ┗┳┓┏╋━━┓ ┣╋┫┃┃┗┻┓┣━┻┳┓┏╋┛ ┣┳┳┛  ┃┣┻┫┏┫  ┃┗┫┗┻┳┛ ┣━╋━━━╋┳┳┛┃┏━┓┏┳╋┫┗┛ ┃┏┛┃ 
┏┳┻┛┣┳┓┃┃┣┛┏━┛┏┫┃┣╋╋┳┳┫┃┏┓┣╋┛┗┓ ┣┻┻┳━━╋╋┓┗┛┣┓ ┣┓┣━┓┃  ┣━╋┓  ┣┫┗┓┃┗┳╋┛┃┣╋━┓┏┻╋┓┗┓
┻╋┳━┫┗┻┫┗╋━┫┏┳╋╋╋┫┣┫┗┛┗┻╋┛┣┻┳━┻━┫  ┃┏┳┫┗╋━━┻┫ ┃┣┫ ┃┗┳━┻━┛┣┓┏╋┫ ┣┻━┫┣━┛┗╋━┛┣┳┻┛ ┣
┳╋┻┳╋━┳┫ ┃ ┣┻┫┗╋┫┃┗┛    ┣━┻━┻┳━┳┛┏━╋╋╋┫ ┃   ┣━┻┻┛ ┣┓┗━┳━┳┛┃┣┫┃ ┣━┳┛┃┏┓ ┣┳━┛┣┓  ┣
┫┣┓┃┗┳┻┫ ┣┳╋┳╋┳┫┗╋┳━┓   ┣┳┓┏━┫ ┣┳┫ ┣┫┃┃ ┃┏┳┓┃┏┓┏┓ ┃┣━┓┗┳┫ ┃┣┫┃┏┫┏┫ ┃┗┻┓┃┗┳┓┣┻━┳┛
┫┣╋┻┓┣┓┗┓┗┫┃┗┫┗╋━╋┫ ┃┏┳━┻┫┗┫ ┣┳┛┃┣┳┻┻┻╋━┫┃┃┣┫┗╋┫┗┓┣┫┏╋┳╋┫ ┃┣╋╋┛┃┣┻┓┗━┓┃┣┓┃┣┻┳┓┗━
┗┫┗┓┗╋╋┳╋━┻┫┏┻━┫ ┃┃┏┫┃┣┓┏┻┓┃ ┃┣┳┻┛┗┳┳━┻━╋┛┗╋┛┏╋╋━╋╋┛┃┣╋╋┫┏┫┗╋┫ ┃┗┓┗━┳╋┛┗┫┣┻┓┃┗━┳
┓┣┳┛┏╋┫┃┣━━┻┻┳┳╋┳┛┣┫┗╋┫┗┻┳┻┻┓┣╋┛   ┃┗┳┓┏┛┏━┛ ┣┫┗━┛┃ ┃┃┃┣╋┛┣┓┃┗┓┣━┫┏┳┛┣┓┏┫┃ ┣╋┳━╋
┫┗┛┏┛┗┫┣╋┳┳┳━┛┃┣┻┳┛┃┏┫┃  ┣┳┓┃┗┛┏━┳┳╋━┛┗╋━╋━━┓┗┛   ┣━╋┫┃┃┣━┫┣┻━╋╋━┫┣┻━┛┃┗╋┫┏┛┃┗┳┛
┫  ┗┓┏┫┣┻┻┛┗┳┳╋┫ ┗┳┻┻╋╋━━┻┫┗┻━┳┫┏╋╋╋━┳━┫ ┃┏━┻━━┳┳┓┣━┻┫┣╋┛ ┣╋━┳┫┗┓┃┣━┓ ┣┓┗┻┫┏┻━┻┓
┃ ┏━┛┃┃┃    ┗┻╋┫ ┏┛  ┣╋┳┓ ┣┳━┓┣╋┛┃┣┻┳┻┓┃┏┻┻━━┓┏╋┛┃┃┏━╋┫┣┓┏╋╋━┫┣┳┛┃┗┳╋┳┻╋━━┫┃   ┣
┣┳╋━┓┗┛┃    ┏━┫┃ ┣━┳┳┻┛┗┛ ┣╋┓┗┻┻┳┛┣┳┫┏┻╋┫    ┃┗┫ ┣╋╋━┫┗┫┗┻┫┗┳╋┫┗┓┗┳╋╋┛┏╋┓ ┣┫   ┣
┗╋┫ ┃  ┃    ┣┓┗╋┓┣┓┣┫     ┃┣┫┏┳━┛ ┣╋┛┃┏╋┛┏┳┓┏┛ ┣┳╋╋┛ ┃ ┣┓┏┫ ┣╋╋┓┗┳╋╋╋━┻╋┫┏┛┗━┳┓┗
┓┃┣┓┃  ┃    ┃┃┏╋╋╋╋┛┃     ┃┣┻┻╋┓┏┓┃┣┓┃┗┛┏╋┛┗┫┏━┻┛┃┗┓ ┃ ┃┗┻╋┳┻┻┫┃┏╋╋┫┣┳━╋┫┗━┳┳┛┗━
┗╋╋╋┛┏━┫ ┏━┳┻╋┻┫┃┃┣━┛     ┣┻┓┏╋╋┛┗┻┛┣┫  ┃┣┳┳┛┣┳┓┏┻┳┫ ┃ ┣┓ ┃┃  ┣╋┛┃┗┫┃┣┳╋╋┳┳┫┗┳┳┓
┳┫┗┫ ┃┏┻━╋━┛┏╋┓┃┃┗┛┏┓     ┣┳╋╋╋┛┏━━┓┃┃  ┃┃┃┃ ┣┫┣╋┓┃┃ ┃┏╋╋━┻┫┏┳┛┗┓┗┳╋┛┃┣┻┫┗┛┣━┻╋╋
┫┃┏┻┓┗┫ ┏┫  ┣┛┣┻┫  ┣┫     ┣┻┻┻┻┳┻┳┳┫┃┃┏┓┗┻┻┻━╋┛┣┫┗┛┃┏╋┫┃┣┓┏┛┗┛┏┳┻┓┗┫ ┃┗━╋━┓┃  ┃┗
┛┗┛┏┻┓┗━┛┗━┓┃ ┗┓┗┳┓┃┃┏━┓  ┃┏━┳┓┃ ┣╋┛┃┗┻╋┓┏┳┓┏┻┳╋┫┏┳┛┗╋┛┣╋┛┗┓┏┳┛┗┓┣━┻┳┫┏━┛┏┫┣┓┏┻┳
  ┏┫ ┃     ┗┛┏┳╋━╋┛┣┻╋┳┛  ┃┗┓┃┣╋┓┃┗┓┃  ┣┫┣┻┫┃┏╋┫┣╋╋┓┏╋━╋┫  ┃┣┫┏┓┃┃  ┃┃┗┓┏┻┛┃┣┫ ┃
┏┓┗┛┏┫       ┗┫┗┓┗┳╋┓┃┗━┳┓┗┳╋┫┣╋╋╋┳┫┣┳━┫┣╋━┻┛┗┫┃┃┗┻╋┛┣┳┫┣┓ ┃┃┃┃┗┻╋━┳┛┃ ┃┃┏┳┻┫┃ ┣
┣┻┳┓┗┫        ┃ ┃┏┻┛┗┫  ┃┣┳┫┣╋┛┗╋┫┣┫┗┻┓┗╋╋┳┳┳━╋┛┣┓ ┣┳╋╋┛┣┻┳┛┃┗┫  ┃┏┛┏╋━┛┗╋┛┏┫┃┏┻
┻┓┗┫┏┛┏┓      ┃┏┛┃   ┣━━┫┣┫┣┫┗┓┏╋╋╋┛  ┣━┻┛┃┗┛ ┃ ┃┗┳┛┣┫┃ ┃ ┣┳┫ ┣━┓┗┫┏╋╋┓┏┓┗━┛┗┻┻┳
 ┗━┻┫ ┃┃    ┏━╋╋━╋━┓┏┻━┳┛┃┃┃┗┓┗┛┃┣┫┏┳┓┣┳┳━┛┏━┳┛┏┫ ┃┏┛┣┫ ┣┳╋┛┗┳┻┳┻┳╋┛┃┣┫┃┗┳━┳┓┏┳┫
┓┏┓┏╋┓┗┻━┓┏━╋┳╋┻━╋┓┗┫  ┃┏╋┛┗┓┃  ┣┻┫┗╋┛┣╋╋┓┏┛ ┃ ┣┻┓┃┗┳┻┫ ┣╋╋━┓┃ ┣━╋┫ ┣┫┣┻━┛┏┫┣┫┗┫
┃┃┗┛┗┫   ┗┻━┻┻╋┓┏┫┃ ┃┏┓┃┗┻┓┏┛┣┳━┻┳╋━┫┏╋╋┛┗┫┏┓┃┏╋━┫┃ ┃ ┣━┫┃┃ ┃┣┓┃ ┃┗━╋┫┣┳━┳┛┗┻┛┏╋
┃┃   ┃    ┏┳━━┻┛┗╋╋┳┫┃┣╋┓┏┛┗━╋┛  ┣┻━┫┃┗┫  ┃┣╋╋┫┗┓┃┗┳┻┓┗┳┫┗┛┏╋┛┃┃ ┣┓ ┣┻┻┻━┻┳━┓┏┫┗
┛┣━┳┓┣┓   ┣┻┓┏┳┳━┛┃┗╋┫┗╋┫┗┳━┓┗┳┳┓┃  ┃┃ ┣━┳┛┃┗┻╋━┻┻┓┗┓┣┳┻┻━┓┃┗┓┃┃┏┛┣┳┛     ┃┏╋┫┃┏
┏╋┳╋┫┃┣┳━┳┻┓┣┫┃┃  ┣━┛┃ ┣┛┏┫ ┃┏┛┃┗╋━━┫┗┓┗━┛┏┛┏┳┻┓┏┓┃ ┣╋┫   ┃┃┏┻╋╋╋┓┃┣┓┏┓   ┣┫┣┫┣┫
┛┃┣┛┃┃┣┻┓┣━┛┗┫┃┣┳┓┃  ┗━┻┓┗┻━┫┣┳╋┓┗┓┏┻┓┗┳┓┏┛┏┫┗━╋┛┃┗┓┗┻╋┳┓┏┫┃┣━┻╋┫┣╋╋┫┣┻┳┳┓┗┻┫┗╋╋
 ┗╋┓┣┫┃ ┗┻┳━┳┫┗┛┃┗┫   ┏┓┃┏━━┛┗┫┣╋━┻┫┏┛ ┃┗╋━┻┻━┳┫┏┛┏┫┏┓┃┃┗┛┗┫┗┓ ┃┗┻╋╋┫┗┓┗╋╋┓ ┃┏╋╋
 ┏╋┫┗╋┫┏┳┓┣┳┫┃  ┣┓┣┓┏┓┃┃┃┃    ┗┫┣┳┳╋┛┏┓┗┓┃    ┣┻┻┳┻┛┣┫┣┻━━━┛┏┛┏╋┓┏┻╋┛ ┃ ┣╋╋━┛┣┻┫
┳╋┻┫┏┻┫┗┫┃┣╋┫┣━┳┻┛┣┫┗┛┣┫┗┫    ┏┛┣╋┛┣┓┃┣┓┗┫    ┃  ┗┳┓┣┛┣━━┳┳━┛┏┛┗┫┃ ┃  ┃┏┻┛┃  ┣┳┻
┃┃ ┣╋━┻┓┣┛┣┻┫┗┓┃ ┏┫┃  ┣╋┓┃┏━━┳┫ ┃┃ ┣┫┣╋┻┓┣┳━┳━┛   ┣┫┗━┻━━┛┗┓┏┛  ┃┃┏╋┓ ┣╋┓┏┛  ┣┻┳
┻┛┏┛┣┳┓┃┗━┻━┻┓┣┫┏┛┃┃  ┗┛┃┗╋┳┳┫┗┓┗╋┳╋┫┃┃ ┣┫┗┳┫     ┃┃       ┃┣┓  ┃┃┗┻┛┏┻╋╋┛┏┳┓┣┓┃
┓ ┣┓┃┃┣╋┳┓┏┓┏┻╋┛┗━┛┗┓   ┣┓┣┫┗┛ ┣━┻┻╋┫┣┻━┫┣┳┫┣━┓┏━┳╋╋━┓┏━┳┳━╋┫┗━┓┃┣┳┓┏┛┏╋╋┳┛┃┗╋┫┃
```
