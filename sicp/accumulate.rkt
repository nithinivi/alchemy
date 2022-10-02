;; 

#lang racket

(define (accumulate combiner init term a next b)
  (define (iter a result)
    (if (< a b)
        result
        (iter (next a ) (combiner result (term a)))))
  (iter a init)
  )

(define (product term a next b) ( accumulate * 1 term a next b))
(define (sum     term a next b) ( accumulate + 0 term a next b))
