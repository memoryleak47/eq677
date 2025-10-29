cnf(a,axiom, X = f(Y, f(X, f(f(Y, X), Y)))).
cnf(a,axiom, X = f(f(Y, X), f(f(Y, f(Y, X)), Y))).

% We are considering the self-producing cycle C(a,a).
% We have
% - the start a*a = s.
% - the end a*e = a.
% - the next-to-last a*n = e.

cnf(a,axiom, f(a,a) = s).
cnf(a,axiom, f(a,n) = e).
cnf(a,axiom, f(a,e) = a).

% Goals:
% cnf(a,axiom, f(s,a) != n).
% cnf(a,axiom, f(e,s) != n).
