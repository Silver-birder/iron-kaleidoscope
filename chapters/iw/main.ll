; ModuleID = 'fizzbuzz'
source_filename = "fizzbuzz"

@fizzbuzz.1 = private unnamed_addr constant [10 x i8] c"fizzbuzz\0A\00", align 1
@fizz = private unnamed_addr constant [6 x i8] c"fizz\0A\00", align 1
@buzz = private unnamed_addr constant [6 x i8] c"buzz\0A\00", align 1

declare void @printf(i8*, ...)

define i64 @fizzbuzz(i64 %0) {
entry:
  %rem = srem i64 %0, 3
  %rem1 = srem i64 %0, 5
  %rem2 = srem i64 %0, 15
  %if = icmp eq i64 %rem, 0
  %if3 = icmp eq i64 %rem1, 0
  %if4 = icmp eq i64 %rem2, 0
  br i1 %if4, label %fb_then, label %con_1

fb_then:                                          ; preds = %entry
  call void (i8*, ...) @printf(i8* getelementptr inbounds ([10 x i8], [10 x i8]* @fizzbuzz.1, i32 0, i32 0))
  br label %ifcont

con_1:                                            ; preds = %entry
  br i1 %if, label %f_else_if, label %con_2

con_2:                                            ; preds = %con_1
  br i1 %if3, label %b_else, label %ifcont

f_else_if:                                        ; preds = %con_1
  call void (i8*, ...) @printf(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @fizz, i32 0, i32 0))
  br label %ifcont

b_else:                                           ; preds = %con_2
  call void (i8*, ...) @printf(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @buzz, i32 0, i32 0))
  br label %ifcont

ifcont:                                           ; preds = %b_else, %con_2, %f_else_if, %fb_then
  ret i8* null
}
