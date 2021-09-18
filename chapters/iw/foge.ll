; ModuleID = 'main'
source_filename = "main"

@hw = private unnamed_addr constant [14 x i8] c"Hello, world!\00", align 1

declare i32 @printf(i8*, ...)

define i32 @main() {
entry:
  %call = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([14 x i8], [14 x i8]* @hw, i32 0, i32 0))
  ret i32 0
}
