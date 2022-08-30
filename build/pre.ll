; ModuleID = 'main'
source_filename = "main"

define i8 @main() {
entry:
  %pemdas = alloca i8, align 1
  store i8 5, i8* %pemdas, align 1
  %pemdas1 = load i8, i8* %pemdas, align 1
  %tmp = urem i8 %pemdas1, 3
  %tmp2 = icmp eq i8 %tmp, 0
  %tmp3 = sext i1 %tmp2 to i8
  %tmp4 = icmp ne i8 %tmp3, 0
  br i1 %tmp4, label %then, label %else

then:                                             ; preds = %entry
  store i8 2, i8* %pemdas, align 1
  br label %end

else:                                             ; preds = %entry
  %pemdas5 = load i8, i8* %pemdas, align 1
  %tmp6 = urem i8 %pemdas5, 5
  %tmp7 = icmp eq i8 %tmp6, 0
  %tmp8 = sext i1 %tmp7 to i8
  %tmp9 = icmp ne i8 %tmp8, 0
  br i1 %tmp9, label %then10, label %else11

end:                                              ; preds = %end12, %then
  %pemdas_f = alloca i8, align 1
  %pemdas13 = load i8, i8* %pemdas, align 1
  store i8 %pemdas13, i8* %pemdas_f, align 1
  %pemdas14 = load i8, i8* %pemdas, align 1
  %pemdas_f15 = load i8, i8* %pemdas_f, align 1
  %tmp16 = add i8 %pemdas_f15, 4
  %tmp17 = icmp ne i8 %pemdas14, %tmp16
  %tmp18 = sext i1 %tmp17 to i8
  %tmp19 = icmp ne i8 %tmp18, 0
  br i1 %tmp19, label %loop, label %end20

then10:                                           ; preds = %else
  store i8 4, i8* %pemdas, align 1
  br label %end12

else11:                                           ; preds = %else
  store i8 6, i8* %pemdas, align 1
  br label %end12

end12:                                            ; preds = %else11, %then10
  br label %end

loop:                                             ; preds = %loop, %end
  %pemdas21 = load i8, i8* %pemdas, align 1
  %tmp22 = add i8 %pemdas21, 1
  store i8 %tmp22, i8* %pemdas, align 1
  %pemdas23 = load i8, i8* %pemdas, align 1
  %pemdas_f24 = load i8, i8* %pemdas_f, align 1
  %tmp25 = add i8 %pemdas_f24, 4
  %tmp26 = icmp ne i8 %pemdas23, %tmp25
  %tmp27 = sext i1 %tmp26 to i8
  %tmp28 = icmp ne i8 %tmp27, 0
  br i1 %tmp28, label %loop, label %end20

end20:                                            ; preds = %loop, %end
  %pemdas29 = load i8, i8* %pemdas, align 1
  ret i8 %pemdas29
}
