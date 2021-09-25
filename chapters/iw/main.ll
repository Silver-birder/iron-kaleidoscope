; ModuleID = 'main'
source_filename = "main"

define i64 @main(i64 %0, i64 %1) {
entry:
  %sum = add i64 %0, %1
  store i64 %sum, i64* null, align 4
  ret i64 %sum
}
