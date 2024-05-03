\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{puthesis}
\RequirePackage{setspace}
\RequirePackage{xcolor}
\def\current@color{ Black}
\newcounter{subyear}
\setcounter{subyear}{\number\year}
\def\submitted#1{\gdef\@submitted{#1}}
\def\@submittedyear{\ifnum\month>10 \stepcounter{subyear}\thesubyear
  \else\thesubyear\fi}
\def\@submittedmonth{\ifnum\month>10 January\else\ifnum\month>8 November
  \else\ifnum\month>6 September\else May\fi\fi\fi}
\def\adviser#1{\gdef\@adviser{#1}}
\long\def\@abstract{\@latex@error{No \noexpand\abstract given}\@ehc}
\newcommand*{\frontmatter}{
  %\pagenumbering{roman}
  }
\newcommand*{\mainmatter}{
  %\pagenumbering{arabic}
}
\newcommand*{\makelot}{}
\newcommand*{\makelof}{}
\newcommand*{\makelos}{}
\newcommand*{\begincmd}{
\doublespacing
\frontmatter\maketitlepage\makecopyrightpage\makeabstract
\makeacknowledgments\makededication\tableofcontents\clearpage
\makelot\clearpage\makelof\clearpage\makelos
\clearpage\mainmatter}
\def\@submitted{\@submittedmonth~\@submittedyear}
\def\@dept{Operations Research and Financial Engineering}
\def\@deptpref{Department of}
\def\departmentprefix#1{\gdef\@deptpref{#1}}
\def\department#1{\gdef\@dept{#1}}
\long\def\acknowledgments#1{\gdef\@acknowledgments{#1}}
\def\dedication#1{\gdef\@dedication{#1}}
\newcommand{\maketitlepage}{{
  \thispagestyle{empty}
  \sc
  \vspace*{0in}
  \begin{center}
    \LARGE \@title
  \end{center}
  \vspace{.6in}
  \begin{center}
    \@author
  \end{center}
  \vspace{.6in}
  \begin{center}
    A Dissertation \\
    Presented to the Faculty \\
    of Princeton University \\
    in Candidacy for the Degree \\
    of Doctor of Philosophy
  \end{center}
  \vspace{.3in}
  \begin{center}
    Recommended for Acceptance \\
    by the \@deptpref \\
    \@dept \\
    Adviser: \@adviser
  \end{center}
  \vspace{.3in}
  \begin{center}
    \@submitted
  \end{center}
  \clearpage
  }}
\newcommand*{\makecopyrightpage}{
  \thispagestyle{empty}
  \vspace*{0in}
  \begin{center}
    \copyright\ Copyright by \@author, \number\year. \\
    All rights reserved.
  \end{center}
  \clearpage}
\newcommand*{\makeabstract}{
  \newpage
  \addcontentsline{toc}{section}{Abstract}
  \begin{center}
  \Large \textbf{Abstract}
  \end{center}
  \@abstract
  \clearpage
  }
\def\makeacknowledgments{
  \ifx\@acknowledgments\undefined
  \else
    \addcontentsline{toc}{section}{Acknowledgments}
    \begin{center}
      \Large \textbf{Acknowledgments}
    \end{center}
    \@acknowledgments
    \clearpage
  \fi
  }
\def\makededication{
  \ifx\@dedication\undefined
  \else
    \vspace*{1.5in}
    \begin{flushright}
      \@dedication
    \end{flushright}
    \clearpage
  \fi
  }
\DeclareOption{myorder}{
  \renewcommand*{\begincmd}{\doublespacing}}
\DeclareOption{lot}{\renewcommand*{\makelot}{
  \addcontentsline{toc}{section}{List of Tables}\listoftables}}
\DeclareOption{lof}{\renewcommand*{\makelof}{
  \addcontentsline{toc}{section}{List of Figures and Tables}\listoffigures}}
\DeclareOption{los}{
  \renewcommand*{\makelos}{
    \RequirePackage{losymbol}
    \section*{List of Symbols\@mkboth {LIST OF SYMBOLS}{LIST OF SYMBOLS}}
    \@starttoc{los}
    \addcontentsline{toc}{section}{List of Symbols}
  }
}
\DeclareOption*{\PassOptionsToClass{\CurrentOption}{report}}
\ProcessOptions
\LoadClass{report}
\setlength{\oddsidemargin}{0.2in}
\setlength{\evensidemargin}{0.2in}
\setlength{\topmargin}{0in}
\setlength{\headheight}{0in}
\setlength{\headsep}{0in}
\setlength{\textheight}{8.9in}
\setlength{\textwidth}{6.1in}
\setlength{\footskip}{0.5in}
\long\def\abstract#1{\gdef\@abstract{#1}}
\AtBeginDocument{\begincmd}
\endinput
