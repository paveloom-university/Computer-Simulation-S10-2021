initSidebarItems({"fn":[["HH_solve","This function solves the system A x = b directly using Householder transformations. On output the solution is stored in x and b is not modified. The matrix A is destroyed by the Householder transformations."],["HH_svx","This function solves the system A x = b in-place using Householder transformations. On input x should contain the right-hand side b, which is replaced by the solution on output. The matrix A is destroyed by the Householder transformations."],["LU_decomp","Factorise a general N x N matrix A into,"],["LU_det","This function computes the determinant of a matrix A from its LU decomposition, LU. The determinant is computed as the product of the diagonal elements of U and the sign of the row permutation signum."],["LU_invert","This function computes the inverse of a matrix A from its LU decomposition (LU,p), storing the result in the matrix inverse. The inverse is computed by solving the system A x = b for each column of the identity matrix. It is preferable to avoid direct use of the inverse whenever possible, as the linear solver functions can obtain the same result more efficiently and reliably (consult any introductory textbook on numerical linear algebra for details)."],["LU_lndet","These functions compute the logarithm of the absolute value of the determinant of a matrix A, \\ln|\\det(A)|, from its LU decomposition, LU. This function may be useful if the direct computation of the determinant would overflow or underflow."],["LU_refine","This function applies an iterative improvement to x, the solution of A x = b, from the precomputed LU decomposition of A into (LU,p). The initial residual r = A x - b is also computed and stored in residual."],["LU_sgndet","This function computes the sign or phase factor of the determinant of a matrix A, \\det(A)/|\\det(A)|, from its LU decomposition, LU."],["LU_solve","This function solves the square system A x = b using the LU decomposition of A into (LU, p) given by LU_decomp or LU_decomp as input."],["LU_svx","This function solves the square system A x = b in-place using the precomputed LU decomposition of A into (LU,p). On input x should contain the right-hand side b, which is replaced by the solution on output."],["QRPT_QRsolve","This function solves the square system R P^T x = Q^T b for x. It can be used when the QR decomposition of a matrix is available in unpacked form as (Q, R)."],["QRPT_Rsolve","This function solves the triangular system R P^T x = b for the N-by-N matrix R contained in QR."],["QRPT_Rsvx","This function solves the triangular system R P^T x = b in-place for the N-by-N matrix R contained in QR. On input x should contain the right-hand side b, which is replaced by the solution on output."],["QRPT_decomp","This function factorizes the M-by-N matrix A into the QRP^T decomposition A = Q R P^T. On output the diagonal and upper triangular part of the input matrix contain the matrix R. The permutation matrix P is stored in the permutation p. The sign of the permutation is given by signum. It has the value (-1)^n, where n is the number of interchanges in the permutation. The vector tau and the columns of the lower triangular part of the matrix A contain the Householder coefficients and vectors which encode the orthogonal matrix Q. The vector tau must be of length k=\\min(M,N). The matrix Q is related to these components by, Q = Q_k … Q_2 Q_1 where Q_i = I - \\tau_i v_i v_i^T and v_i is the Householder vector v_i = (0,…,1,A(i+1,i),A(i+2,i),…,A(m,i)). This is the same storage scheme as used by LAPACK. The vector norm is a workspace of length N used for column pivoting."],["QRPT_decomp2","This function factorizes the matrix A into the decomposition A = Q R P^T without modifying A itself and storing the output in the separate matrices q and r."],["QRPT_solve","This function solves the square system A x = b using the QRP^T decomposition of A held in (QR, tau, p) which must have been computed previously by QRPT_decomp."],["QRPT_svx","This function solves the square system A x = b in-place using the QRP^T decomposition of A held in (QR,tau,p). On input x should contain the right-hand side b, which is replaced by the solution on output."],["QRPT_update","This function performs a rank-1 update w v^T of the QRP^T decomposition (Q, R, p). The update is given by Q’R’ = Q (R + w v^T P) where the output matrices Q’ and R’ are also orthogonal and right triangular. Note that w is destroyed by the update. The permutation p is not changed."],["QR_QRsolve","This function solves the system R x = Q^T b for x. It can be used when the QR decomposition of a matrix is available in unpacked form as (Q, R)."],["QR_QTmat","This function applies the matrix Q^T encoded in the decomposition (QR,tau) to the matrix A, storing the result Q^T A in A. The matrix multiplication is carried out directly using the encoding of the Householder vectors without needing to form the full matrix Q^T."],["QR_QTvec","This function applies the matrix Q^T encoded in the decomposition (QR,tau) to the vector v, storing the result Q^T v in v. The matrix multiplication is carried out directly using the encoding of the Householder vectors without needing to form the full matrix Q^T."],["QR_Qvec","This function applies the matrix Q encoded in the decomposition (QR,tau) to the vector v, storing the result Q v in v. The matrix multiplication is carried out directly using the encoding of the Householder vectors without needing to form the full matrix Q."],["QR_Rsolve","This function solves the triangular system R x = b for x. It may be useful if the product b’ = Q^T b has already been computed using gsl_linalg_QR_QTvec."],["QR_Rsvx","This function solves the triangular system R x = b for x in-place. On input x should contain the right-hand side b and is replaced by the solution on output. This function may be useful if the product b’ = Q^T b has already been computed using gsl_linalg_QR_QTvec."],["QR_decomp","This function factorizes the M-by-N matrix A into the QR decomposition A = Q R. On output the diagonal and upper triangular part of the input matrix contain the matrix R. The vector tau and the columns of the lower triangular part of the matrix A contain the Householder coefficients and Householder vectors which encode the orthogonal matrix Q. The vector tau must be of length k=\\min(M,N). The matrix Q is related to these components by, Q = Q_k … Q_2 Q_1 where Q_i = I - \\tau_i v_i v_i^T and v_i is the Householder vector v_i = (0,…,1,A(i+1,i),A(i+2,i),…,A(m,i)). This is the same storage scheme as used by LAPACK."],["QR_lssolve","This function finds the least squares solution to the overdetermined system A x = b where the matrix A has more rows than columns. The least squares solution minimizes the Euclidean norm of the residual, ||Ax - b||.The routine requires as input the QR decomposition of A into (QR, tau) given by gsl_linalg_QR_decomp. The solution is returned in x. The residual is computed as a by-product and stored in residual."],["QR_solve","This function solves the square system A x = b using the QR decomposition of A held in (QR, tau) which must have been computed previously with gsl_linalg_QR_decomp. The least-squares solution for rectangular systems can be found using QR_lssolve."],["QR_svx","This function solves the square system A x = b in-place using the QR decomposition of A held in (QR,tau) which must have been computed previously by gsl_linalg_QR_decomp. On input x should contain the right-hand side b, which is replaced by the solution on output."],["QR_unpack","This function unpacks the encoded QR decomposition (QR,tau) into the matrices Q and R, where Q is M-by-M and R is M-by-N."],["QR_update","This function performs a rank-1 update w v^T of the QR decomposition (Q, R). The update is given by Q’R’ = Q (R + w v^T) where the output matrices Q’ and R’ are also orthogonal and right triangular. Note that w is destroyed by the update."],["R_solve","This function solves the triangular system R x = b for the N-by-N matrix R."],["R_svx","This function solves the triangular system R x = b in-place. On input x should contain the right-hand side b, which is replaced by the solution on output."],["SV_decomp","This function factorizes the M-by-N matrix A into the singular value decomposition A = U S V^T for M >= N. On output the matrix A is replaced by U. The diagonal elements of the singular value matrix S are stored in the vector S. The singular values are non-negative and form a non-increasing sequence from S_1 to S_N. The matrix V contains the elements of V in untransposed form. To form the product U S V^T it is necessary to take the transpose of V. A workspace of length N is required in work."],["SV_decomp_jacobi","This function computes the SVD of the M-by-N matrix A using one-sided Jacobi orthogonalization for M >= N. The Jacobi method can compute singular values to higher relative accuracy than Golub-Reinsch algorithms (see references for details)."],["SV_decomp_mod","This function computes the SVD using the modified Golub-Reinsch algorithm, which is faster for M>>N. It requires the vector work of length N and the N-by-N matrix X as additional working space."],["SV_leverage","This function computes the statistical leverage values h_i of a matrix A using its singular value decomposition (U, S, V) previously computed with gsl_linalg_SV_decomp. h_i are the diagonal values of the matrix A (A^T A)^{-1} A^T and depend only on the matrix U which is the input to this function."],["SV_solve","This function solves the system A x = b using the singular value decomposition (U, S, V) of A which must have been computed previously with gsl_linalg_SV_decomp."],["balance_matrix","This function replaces the matrix A with its balanced counterpart and stores the diagonal elements of the similarity transformation into the vector D."],["bidiag_decomp","This function factorizes the M-by-N matrix A into bidiagonal form U B V^T. The diagonal and superdiagonal of the matrix B are stored in the diagonal and superdiagonal of A. The orthogonal matrices U and V are stored as compressed Householder vectors in the remaining elements of A. The Householder coefficients are stored in the vectors tau_U and tau_V. The length of tau_U must equal the number of elements in the diagonal of A and the length of tau_V should be one element shorter."],["bidiag_unpack","This function unpacks the bidiagonal decomposition of A produced by gsl_linalg_bidiag_decomp, (A, tau_U, tau_V) into the separate orthogonal matrices U, V and the diagonal vector diag and superdiagonal superdiag. Note that U is stored as a compact M-by-N orthogonal matrix satisfying U^T U = I for efficiency."],["bidiag_unpack2","This function unpacks the bidiagonal decomposition of A produced by gsl_linalg_bidiag_decomp, (A, tau_U, tau_V) into the separate orthogonal matrices U, V and the diagonal vector diag and superdiagonal superdiag. The matrix U is stored in-place in A."],["bidiag_unpack_B","This function unpacks the diagonal and superdiagonal of the bidiagonal decomposition of A from gsl_linalg_bidiag_decomp, into the diagonal vector diag and superdiagonal vector superdiag."],["cholesky_decomp","This function factorizes the symmetric, positive-definite square matrix A into the Cholesky decomposition A = L L^T (or A = L L^H for the complex case). On input, the values from the diagonal and lower-triangular part of the matrix A are used (the upper triangular part is ignored). On output the diagonal and lower triangular part of the input matrix A contain the matrix L, while the upper triangular part of the input matrix is overwritten with L^T (the diagonal terms being identical for both L and L^T). If the matrix is not positive-definite then the decomposition will fail, returning the error code ::Dom."],["cholesky_invert","This function computes the inverse of a matrix from its Cholesky decomposition cholesky, which must have been previously computed by gsl_linalg_cholesky_decomp or gsl_linalg_complex_cholesky_decomp. On output, the inverse is stored in-place in cholesky."],["cholesky_solve","This function solves the system A x = b using the Cholesky decomposition of A held in the matrix cholesky which must have been previously computed by gsl_linalg_cholesky_decomp or gsl_linalg_complex_cholesky_decomp."],["cholesky_svx","This function solves the system A x = b in-place using the Cholesky decomposition of A held in the matrix cholesky which must have been previously computed by gsl_linalg_cholesky_decomp or gsl_linalg_complex_cholesky_decomp. On input x should contain the right-hand side b, which is replaced by the solution on output."],["complex_LU_decomp","Factorise a general N x N complex matrix A into,"],["complex_LU_det","This function computes the determinant of a matrix A from its LU decomposition, LU. The determinant is computed as the product of the diagonal elements of U and the sign of the row permutation signum."],["complex_LU_invert","This function computes the inverse of a matrix A from its LU decomposition (LU,p), storing the result in the matrix inverse. The inverse is computed by solving the system A x = b for each column of the identity matrix. It is preferable to avoid direct use of the inverse whenever possible, as the linear solver functions can obtain the same result more efficiently and reliably (consult any introductory textbook on numerical linear algebra for details)."],["complex_LU_lndet","This function computes the sign or phase factor of the determinant of a matrix A, \\det(A)/|\\det(A)|, from its LU decomposition, LU."],["complex_LU_refine","This function applies an iterative improvement to x, the solution of A x = b, from the precomputed LU decomposition of A into (LU,p). The initial residual r = A x - b is also computed and stored in residual."],["complex_LU_sgndet","This function computes the sign or phase factor of the determinant of a matrix A, \\det(A)/|\\det(A)|, from its LU decomposition, LU."],["complex_LU_solve","This function solves the square system A x = b using the LU decomposition of A into (LU, p) given by LU_decomp or LU_decomp as input."],["complex_LU_svx","This function solves the square system A x = b in-place using the precomputed LU decomposition of A into (LU,p). On input x should contain the right-hand side b, which is replaced by the solution on output."],["complex_cholesky_decomp","This function factorizes the symmetric, positive-definite square matrix A into the Cholesky decomposition A = L L^T (or A = L L^H for the complex case). On input, the values from the diagonal and lower-triangular part of the matrix A are used (the upper triangular part is ignored). On output the diagonal and lower triangular part of the input matrix A contain the matrix L, while the upper triangular part of the input matrix is overwritten with L^T (the diagonal terms being identical for both L and L^T). If the matrix is not positive-definite then the decomposition will fail, returning the error code ::Dom."],["complex_cholesky_invert","This function computes the inverse of a matrix from its Cholesky decomposition cholesky, which must have been previously computed by gsl_linalg_cholesky_decomp or gsl_linalg_complex_cholesky_decomp. On output, the inverse is stored in-place in cholesky."],["complex_cholesky_solve","This function solves the system A x = b using the Cholesky decomposition of A held in the matrix cholesky which must have been previously computed by gsl_linalg_cholesky_decomp or gsl_linalg_complex_cholesky_decomp."],["complex_cholesky_svx","This function solves the system A x = b in-place using the Cholesky decomposition of A held in the matrix cholesky which must have been previously computed by gsl_linalg_cholesky_decomp or gsl_linalg_complex_cholesky_decomp. On input x should contain the right-hand side b, which is replaced by the solution on output."],["complex_householder_hm","This function applies the Householder matrix P defined by the scalar tau and the vector v to the left-hand side of the matrix A. On output the result P A is stored in A."],["complex_householder_hv","This function applies the Householder transformation P defined by the scalar tau and the vector v to the vector w. On output the result P w is stored in w."],["complex_householder_mh","This function applies the Householder matrix P defined by the scalar tau and the vector v to the right-hand side of the matrix A. On output the result A P is stored in A."],["complex_householder_transform","This function prepares a Householder transformation P = I - \\tau v v^T which can be used to zero all the elements of the input vector except the first. On output the transformation is stored in the vector v and the scalar \\tau is returned."],["complex_tri_LHL",""],["complex_tri_UL",""],["complex_tri_invert",""],["givens","Returns `(c, s)`."],["givens_gv",""],["hermtd_decomp","This function factorizes the hermitian matrix A into the symmetric tridiagonal decomposition U T U^T. On output the real parts of the diagonal and subdiagonal part of the input matrix A contain the tridiagonal matrix T. The remaining lower triangular part of the input matrix contains the Householder vectors which, together with the Householder coefficients tau, encode the unitary matrix U. This storage scheme is the same as used by LAPACK. The upper triangular part of A and imaginary parts of the diagonal are not referenced."],["hermtd_unpack","This function unpacks the encoded tridiagonal decomposition (A, tau) obtained from gsl_linalg_hermtd_decomp into the unitary matrix U, the real vector of diagonal elements diag and the real vector of subdiagonal elements subdiag."],["hermtd_unpack_T","This function unpacks the diagonal and subdiagonal of the encoded tridiagonal decomposition (A, tau) obtained from the gsl_linalg_hermtd_decomp into the real vectors diag and subdiag."],["hessenberg_decomp","This function computes the Hessenberg decomposition of the matrix A by applying the similarity transformation H = U^T A U. On output, H is stored in the upper portion of A. The information required to construct the matrix U is stored in the lower triangular portion of A. U is a product of N - 2 Householder matrices. The Householder vectors are stored in the lower portion of A (below the subdiagonal) and the Householder coefficients are stored in the vector tau. tau must be of length N."],["hessenberg_set_zero","This function sets the lower triangular portion of H, below the subdiagonal, to zero. It is useful for clearing out the Householder vectors after calling gsl_linalg_hessenberg_decomp."],["hessenberg_unpack","This function constructs the orthogonal matrix U from the information stored in the Hessenberg matrix H along with the vector tau. H and tau are outputs from gsl_linalg_hessenberg_decomp."],["hessenberg_unpack_accum","This function is similar to gsl_linalg_hessenberg_unpack, except it accumulates the matrix U into V, so that V’ = VU. The matrix V must be initialized prior to calling this function. Setting V to the identity matrix provides the same result as gsl_linalg_hessenberg_unpack. If H is order N, then V must have N columns but may have any number of rows."],["hesstri_decomp","This function computes the Hessenberg-Triangular decomposition of the matrix pair (A, B). On output, H is stored in A, and R is stored in B. If U and V are provided (they may be null), the similarity transformations are stored in them. Additional workspace of length N is needed in work."],["householder_hm","This function applies the Householder matrix P defined by the scalar tau and the vector v to the left-hand side of the matrix A. On output the result P A is stored in A."],["householder_hv","This function applies the Householder transformation P defined by the scalar tau and the vector v to the vector w. On output the result P w is stored in w."],["householder_mh","This function applies the Householder matrix P defined by the scalar tau and the vector v to the right-hand side of the matrix A. On output the result A P is stored in A."],["householder_transform","This function prepares a Householder transformation P = I - \\tau v v^T which can be used to zero all the elements of the input vector except the first. On output the transformation is stored in the vector v and the scalar \\tau is returned."],["solve_cyc_tridiag","This function solves the general N-by-N system A x = b where A is cyclic tridiagonal (N >= 3). The cyclic super-diagonal and sub-diagonal vectors e and f must have the same number of elements as the diagonal vector diag. The form of A for the 4-by-4 case is shown below,"],["solve_symm_cyc_tridiag","This function solves the general N-by-N system A x = b where A is symmetric cyclic tridiagonal (N >= 3). The cyclic off-diagonal vector e must have the same number of elements as the diagonal vector diag. The form of A for the 4-by-4 case is shown below,"],["solve_symm_tridiag","This function solves the general N-by-N system A x = b where A is symmetric tridiagonal (N >= 2). The off-diagonal vector e must be one element shorter than the diagonal vector diag. The form of A for the 4-by-4 case is shown below,"],["solve_tridiag","This function solves the general N-by-N system A x = b where A is tridiagonal (N >= 2). The super-diagonal and sub-diagonal vectors e and f must be one element shorter than the diagonal vector diag. The form of A for the 4-by-4 case is shown below,"],["symmtd_decomp","This function factorizes the symmetric square matrix A into the symmetric tridiagonal decomposition Q T Q^T. On output the diagonal and subdiagonal part of the input matrix A contain the tridiagonal matrix T. The remaining lower triangular part of the input matrix contains the Householder vectors which, together with the Householder coefficients tau, encode the orthogonal matrix Q. This storage scheme is the same as used by LAPACK. The upper triangular part of A is not referenced."],["symmtd_unpack","This function unpacks the encoded symmetric tridiagonal decomposition (A, tau) obtained from gsl_linalg_symmtd_decomp into the orthogonal matrix Q, the vector of diagonal elements diag and the vector of subdiagonal elements subdiag."],["symmtd_unpack_T","This function unpacks the diagonal and subdiagonal of the encoded symmetric tridiagonal decomposition (A, tau) obtained from gsl_linalg_symmtd_decomp into the vectors diag and subdiag."]]});