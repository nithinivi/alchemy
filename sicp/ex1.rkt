#lang racket



(define (sum-integers a b)
  (if (> a b)
      0
      (+ a (sum-integers (+ a 1) b)
         )))

(define (sum-cube a b)
  (if ( > a b)
      0
      (+ (cube a) (sum-cube (+ a 1) b))))


(define (cube a) (* a a a))


;; these extremely similar
;; so can be abstracted

(define (sum a trans b next)
  (if (> a b)
      0
      (+ (trans a)
         (sum trans (next a) next b))))

(define (inc a) (+ 1 a))
(define (identity a) a)

(define (sum-cube-n     a b)  (sum a cube     b inc))
(define (sum-interger-n a b)  (sum a identity b inc))


;; sum with linear recursion


(define (suml a trans b next)
  (define (iter a result)
    (if (> a b)
        result
        (iter (next a) (+ result (trans a)))))
  (iter a 0)
  )

;; > (suml 0 identity 10 inc)
;; 55
