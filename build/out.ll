; ModuleID = 'out.bc'
source_filename = "main"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"

define i8 @main() {
entry:
  %pemdas = alloca i8, align 1
  store i8 9, i8* %pemdas, align 1
  %pemdas1 = load i8, i8* %pemdas, align 1
  ret i8 %pemdas1
}
