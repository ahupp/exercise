 (defclass rewindable ()
   ((rewind-store :reader rewind-store
                  :initform (make-array 12 :fill-pointer 0 :adjustable t))
    ;; Index is the number of rewinds we've done.
    (rewind-index :accessor rewind-index
                  :initform 0)))

 (defun rewind-count (rewindable)
   (fill-pointer (rewind-store rewindable)))

 (defun last-state (rewindable)
   (let ((size (rewind-count rewindable)))
     (if (zerop size)
         (values nil nil)
         (values (aref (rewind-store rewindable) (1- size)) t))))

 (defun save-rewindable-state (rewindable object)
   (let ((index (rewind-index rewindable))
         (store (rewind-store rewindable)))
     (unless (zerop index)
       ;; Reverse the tail of pool, since we've
       ;; gotten to the middle by rewinding.
       (setf (subseq store index) (nreverse (subseq store index))))
     (vector-push-extend object store)))

 (defmethod rewind-state ((rewindable rewindable))
   (invariant (not (zerop (rewind-count rewindable))))
   (setf (rewind-index rewindable) 
         (mod (1+ (rewind-index rewindable)) (rewind-count rewindable)))
   (aref (rewind-store rewindable) 
         (- (rewind-count rewindable) (rewind-index rewindable) 1)))

(defun hello ()
(if ())
  (+ 1 1))