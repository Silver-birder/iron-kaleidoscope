; ModuleID = 'fizzbuzz'
source_filename = "fizzbuzz"

@fizzbuzz.1 = private unnamed_addr constant [9 x i8] c"fizzbuzz\00", align 1
@fizz = private unnamed_addr constant [5 x i8] c"fizz\00", align 1
@buzz = private unnamed_addr constant [5 x i8] c"buzz\00", align 1

declare i64 @printf(i8*, ...)

define i64 @fizzbuzz(i64 %0) {
entry:
  %rem = srem i64 %0, 3
  %rem1 = srem i64 %0, 5
  %rem2 = srem i64 %0, 15
  %if = icmp eq i64 %rem, 0
  br i1 %if, label %then, label %else

then:                                             ; preds = %entry
  %then3 = call i64 (i8*, ...) @printf(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @buzz, i32 0, i32 0))
  br label %ifcont

else:                                             ; preds = %entry
  %then4 = call i64 (i8*, ...) @printf(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @fizz, i32 0, i32 0))
  br label %ifcont

ifcont:                                           ; preds = %else, %then
  ret i64 %0
}
