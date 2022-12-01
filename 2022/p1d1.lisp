(defparameter currmax 0)
(defparameter currcalories 0)
(let ((stream (open "input-p1d1.txt" :if-does-not-exist nil)))
    (when stream
        (loop for line = (read-line stream nil)
            while line do 
            (if (string-not-equal line "") 
                (setq currcalories (+ currcalories (parse-integer line)))
                (if (> currcalories currmax) 
                    (progn 
                        (setq currmax currcalories) 
                        (setq currcalories 0)) 
                    (setq currcalories 0))))
        (write currmax)
        (close stream)))