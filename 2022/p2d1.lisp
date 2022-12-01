(defparameter caloriesums (make-array 1000 :adjustable t :fill-pointer 0))
(defparameter currcalories 0)
(let ((stream (open "input-p1d1.txt" :if-does-not-exist nil)))
    (when stream
        (loop for line = (read-line stream nil)
            while line do 
            (if (string-not-equal line "") 
                (setq currcalories (+ currcalories (parse-integer line)))
                (progn
                    (vector-push currcalories caloriesums)
                    (setq currcalories 0)
                    )))
        (sort caloriesums #'>)
        (write (+ (aref caloriesums 0) (aref caloriesums 1) (aref caloriesums 2)))
        (close stream)))