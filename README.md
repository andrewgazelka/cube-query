# cube-query

Query the CubeMX database from the command line.
Currently only supports querying pins.

Example:

```
$ cr -- stm32h743zitx tim1

 Pin           Use               Mode
 PA0           TIM15_BKIN        AF4
 PA1           TIM15_CH1N        AF4
 PA10          TIM1_CH3          AF1
 PA11          TIM1_CH4          AF1
 PA12          TIM1_ETR          AF1
 PA2           TIM15_CH1         AF4
 PA3           TIM15_CH2         AF4
 PA6           TIM13_CH1         AF9
 PA6           TIM1_BKIN         AF1
 PA6           TIM1_BKIN_COMP1   AF12
 PA6           TIM1_BKIN_COMP2   AF12
 PA7           TIM14_CH1         AF9
 PA7           TIM1_CH1N         AF1
 PA8           TIM1_CH1          AF1
 PA9           TIM1_CH2          AF1
 PB0           TIM1_CH2N         AF1
 PB1           TIM1_CH3N         AF1
 PB12          TIM1_BKIN         AF1
 PB12          TIM1_BKIN_COMP1   AF13
 PB12          TIM1_BKIN_COMP2   AF13
 PB13          TIM1_CH1N         AF1
 PB14          TIM12_CH1         AF2
 PB14          TIM1_CH2N         AF1
 PB15          TIM12_CH2         AF2
 PB15          TIM1_CH3N         AF1
 PB4 (NJTRST)  TIM16_BKIN        AF1
 PB5           TIM17_BKIN        AF1
 PB6           TIM16_CH1N        AF1
 PB7           TIM17_CH1N        AF1
 PB8           TIM16_CH1         AF1
 PB9           TIM17_CH1         AF1
 PD12          LPTIM1_IN1        AF1
 PD13          LPTIM1_OUT        AF1
 PE0           LPTIM1_ETR        AF1
 PE1           LPTIM1_IN2        AF1
 PE10          TIM1_CH2N         AF1
 PE11          TIM1_CH2          AF1
 PE12          TIM1_CH3N         AF1
 PE13          TIM1_CH3          AF1
 PE14          TIM1_CH4          AF1
 PE15          TIM1_BKIN         AF1
 PE15          TIM1_BKIN_COMP1   AF13
 PE15          TIM1_BKIN_COMP2   AF13
 PE3           TIM15_BKIN        AF4
 PE4           TIM15_CH1N        AF4
 PE5           TIM15_CH1         AF4
 PE6           TIM15_CH2         AF4
 PE6           TIM1_BKIN2        AF1
 PE6           TIM1_BKIN2_COMP1  AF11
 PE6           TIM1_BKIN2_COMP2  AF11
 PE7           TIM1_ETR          AF1
 PE8           TIM1_CH1N         AF1
 PE9           TIM1_CH1          AF1
 PF10          TIM16_BKIN        AF1
 PF6           TIM16_CH1         AF1
 PF7           TIM17_CH1         AF1
 PF8           TIM13_CH1         AF9
 PF8           TIM16_CH1N        AF1
 PF9           TIM14_CH1         AF9
 PF9           TIM17_CH1N        AF1
 PG11          LPTIM1_IN2        AF1
 PG12          LPTIM1_IN1        AF1
 PG13          LPTIM1_OUT        AF1
 PG14          LPTIM1_ETR        AF1
 PG4           TIM1_BKIN2        AF1
 PG4           TIM1_BKIN2_COMP1  AF11
 PG4           TIM1_BKIN2_COMP2  AF11
 PG5           TIM1_ETR          AF1
 PG6           TIM17_BKIN        AF1
```
