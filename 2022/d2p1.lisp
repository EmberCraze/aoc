(defparameter tot 0)
(let ((stream (open "input-d1.txt" :if-does-not-exist nil)))
    (when stream
        (loop for line = (read-line stream nil)
            while line do 
            (let ((line line))
                (cond ((eql line "A X") (setq tot (+ tot 4))
                      ((eql line "A Y") (setq tot (+ tot 8))
                      ((eql line "B Y") (setq tot (+ tot 5))
                      ((eql line "B Z") (setq tot (+ tot 9))
                      ((eql line "C Z") (setq tot (+ tot 6))
                      ((eql line "C x") (setq tot (+ tot 7))))))))))
        (write tot)
        (close stream)))