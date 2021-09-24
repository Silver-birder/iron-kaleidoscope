; ModuleID = 'tmp'
source_filename = "tmp"

define double @fib(double %n) {
entry:
  %tmpcmp = fcmp ult double %n, 2.000000e+00
  br i1 %tmpcmp, label %ifcont, label %else

else:                                             ; preds = %entry
  %tmpsub = fadd double %n, -1.000000e+00
  %tmp = call double @fib(double %tmpsub)
  %tmpsub6 = fadd double %n, -2.000000e+00
  %tmp7 = call double @fib(double %tmpsub6)
  %tmpadd = fadd double %tmp, %tmp7
  br label %ifcont

ifcont:                                           ; preds = %entry, %else
  %iftmp = phi double [ %tmpadd, %else ], [ %n, %entry ]
  ret double %iftmp
}

define double @anonymous() {
entry:
  %tmp = call double @fib(double 4.000000e+01)
  ret double %tmp
}

; def fib(n) if n < 2 then n else fib(n - 1) + fib(n - 2)
; fib(40) → 102334155