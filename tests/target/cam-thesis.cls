%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%
%% Class ``cam-thesis''
%%
%% Version: v0.2
%% Authors: Jean Martina, Rok Strnisa, Matej Urbas
%% Date: 30/07/2008
%%
%% Copyright (c) 2008-2012, Rok Strniša, Jean Martina, Matej Urbas
%% License: Simplified BSD License
%% License file: ./License
%% Original License URL: http://www.freebsd.org/copyright/freebsd-license.html
%%%%%

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%
%% General guidelines on which this class is based:
%%
%% http://www.cl.cam.ac.uk/local/phd/typography/
%% http://www.admin.cam.ac.uk/offices/gradstud/exams/submission/phd/format.html
%%
%%%%%

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%
%% Class identification.
%%
%%%%%
\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{cam-thesis}[2012/04/12 University of Cambridge thesis class]

\typeout{}
\typeout{***********************************************}
\typeout{***********************************************}
\typeout{}

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%
%% General Cambridge guidelines.
%%
%% LIMIT: 60k words (including tables and footnotes, excluding appendices, bib,
%% photos, diagrams); title and section headings should be capitalized as normal
%% sentences; citations should include authors' initials, and page numbers (if
%% possible); double-sided printing is permissible for the soft bound version;
%% however, single-sided is required for the text of the final, hard bound
%% library copy (diagrams on facing pages are acceptable); always make it
%% possible to create the ps file as well (required for technical reports).
%%
%%%%%

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%
%% Package options (see README.md for a list of options with descriptions).
%%
%% These options can be provided within square brackets of the `documentclass'
%% command.
%%
%%%%%

% techreport - formats the thesis as a technical report.
\newif\ifcam@techreport\cam@techreportfalse
\DeclareOption{techreport}{\cam@techreporttrue}

% times - tells the class to use the times font.
\newif\ifcam@times\cam@timesfalse
\DeclareOption{times}{\cam@timestrue}

% glossary - puts the glossary (after the TOC).
%
\newif\ifcam@glossary\cam@glossaryfalse
\DeclareOption{glossary}{\cam@glossarytrue}

% index - puts the index at the end of the thesis.
%
\newif\ifcam@index\cam@indexfalse
\DeclareOption{withindex}{\cam@indextrue}

% 1st year report - omits abstract/declaration
%
\newif\ifcam@firstyr\cam@firstyrfalse
\DeclareOption{firstyr}{\cam@firstyrtrue}

% 2nd year report - omits declaration
%
\newif\ifcam@secondyr\cam@secondyrfalse
\DeclareOption{secondyr}{\cam@secondyrtrue}

% backrefs - add back references
%
\newif\ifcam@backrefs\cam@backrefsfalse
\DeclareOption{backrefs}{\cam@backrefstrue}

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%
%% Using report class as base.
%%
%%%%%
\PassOptionsToClass{a4paper,12pt,twoside,openright}{report}
\DeclareOption*{\PassOptionsToClass{\CurrentOption}{report}}
\ProcessOptions\relax
\LoadClass{report}

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%
%% For consistent vertical spacing
%%
%%%%%
\raggedbottom

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%
%% Additional packages, and their options.
%%
%%%%%
\RequirePackage{graphicx}           % Required for the UC Logo (on
% the title page)
\RequirePackage{calc}               % Used for calculating margins
% and laying out the title page

% Create the index
\ifcam@index
\RequirePackage{makeidx}
\makeindex
\newcommand{\printthesisindex}{%
  \cleardoublepage%
  \phantomsection%
  \addcontentsline{toc}{chapter}{Index}%
\printindex}
\fi

% Create the glossary
\ifcam@glossary
\RequirePackage{glossaries}
\makeglossaries%
\newcommand{\printthesisglossary}{\printglossary[nonumberlist]}
\newcommand{\cam@printthesisglossary}{%
  \cleardoublepage%
  \pagestyle{empty}%
  \renewcommand{\glossarypreamble}{\thispagestyle{empty}}%
  \printthesisglossary%
}
\fi

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%
%% Page margins (suitable for J.S. Wilson & Son).
%%
%%%%%

\newlength{\cam@topmargin}
\newlength{\cam@bottommargin}
\newlength{\cam@oddmargin}
\newlength{\cam@evenmargin}

%% Calculate and set the margins properly (with parameters that actually have
%% some meaning for everyday thesis-writers).
%%
%% @param  1  odd side margin (inner margin).
%% @param  2  even side margin (outer margin).
%% @param  3  top margin.
%% @param  4  bottom margin.
\DeclareRobustCommand{\cam@calcpaperdims}[4]{%
  % MARGINS
  % 'Top margin' is the distance between the top of the text and the
  % top of the page.
  % 'Bottom margin' is the distance between the bottom of the footer
  % (the page number) and the bottom of the page.
  \setlength{\cam@oddmargin}{#1}        % inner margin
  \setlength{\cam@evenmargin}{#2}       % outer margin
  \setlength{\cam@topmargin}{#3}        % top margin        (the
  % distance from the top of the page to the top of the body text --
  % the header is located between)
  \setlength{\cam@bottommargin}{#4}     % bottom margin     (the
  % distance from the bottom of the page to the bottom of the body
  % text -- the footer is located between)
  % Horizontal spacing
  \setlength{\textwidth}{\paperwidth-\cam@oddmargin-\cam@evenmargin}
  % text takes the remaining width (210 - inner - outer)
  \setlength{\oddsidemargin}{\cam@oddmargin-1in}   % Counter the
  % LaTeX 1in margin
  \setlength{\evensidemargin}{\cam@evenmargin-1in} % Counter the
  % LaTeX 1in margin
  \setlength{\marginparwidth}{\cam@evenmargin-8mm} % the margin only
  % has 'outer' space available, so we have to make it a bit thinner.
  \setlength{\marginparsep}{3mm}

  % Vertical spacing
  \setlength{\headheight}{5mm}      % The height of the box where the
  % heading text lives
  \setlength{\headsep}{5mm}         % The distance between the
  % heading and the top of the text
  \setlength{\topmargin}{\cam@topmargin-\headheight-\headsep-1in} %
  % Counter the LaTeX 1in margin

  \setlength{\textheight}{\paperheight-\cam@topmargin-1.7\cam@bottommargin}
  % text takes the remaining height (297 - top margin - bottom margin)
  \setlength{\footskip}{.7\cam@bottommargin} % The distance from the
  % bottom of the text to the bottom of the footer
}
\ifcam@techreport
\cam@calcpaperdims{25mm}{25mm}{20mm}{20mm}
\else
\cam@calcpaperdims{30mm}{20mm}{20mm}{20mm}
\fi

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%
%% Variable definitions and default values: these variables should be defined by
%% the user (somewhere in the preamble). For example, to put the abstract into
%% the thesis, the thesis writer should type the following somewhere in the
%% preamble (before the `\begin{document}` or `\frontmatter` commands are
%% called):
%%
%%      \abstract{This is my abstract.}
%%
%% See below (in the comments starting with 'DOCVAR: ') for a list of all
% variables
%% the thesis writer is expected to use.
%%
%%%%%

% DOCVAR: abstract (The text that will be inserted into the abstract of the
% thesis.)
\newcommand{\@abstract}{}
\renewcommand{\abstract}[1]{\renewcommand{\@abstract}{#1}}

% DOCVAR: acknowledgements (The text that will be inserted into the
% acknowledgments of the thesis.)
\newcommand{\@acknowledgements}{}
\newcommand{\acknowledgements}[1]{\renewcommand{\@acknowledgements}{#1}}

% DOCVAR: college (The name of the thesis writer's college, which will appear
% just below their name.)
\newcommand{\@college}{}
\newcommand{\college}[1]{\renewcommand{\@college}{#1}}

% DOCVAR: keywords (These keywords will appear in the PDF meta-information
% called `pdfkeywords`.)
\newcommand{\@keywords}{}
\newcommand{\keywords}[1]{\renewcommand{\@keywords}{#1}}

% DOCVAR: subjectline (This subject will appear in the PDF meta-information
% called `pdfsubject`.)
\newcommand{\@subjectline}{}
\newcommand{\subjectline}[1]{\renewcommand{\@subjectline}{#1}}

% DOCVAR: submissiondate (The date of the submission of this thesis. If the
% submission date is provided, it will be printed on the title page--within the
% `submissionnotice` by default. Note that the thesis writer can provide their
% own `submissionnotice`, in which case it is up to them whether they will use
% this date in their notice.)
\newif\ifcam@submissiondate\cam@submissiondatefalse
\newcommand{\@submissiondate}{}
\newcommand{\submissiondate}[1]{%
\renewcommand{\@submissiondate}{#1}\cam@submissiondatetrue}

% DOCVAR: submissionnotice (The submission notice is shown on the bottom of the
% title page.)
\newcommand{\@submissionnotice}{%
  \ifcam@firstyr
  First year report submitted
  \else
  \ifcam@secondyr
  Second year report submitted
  \else
  This dissertation is submitted
  \fi
  \fi
  \ifcam@submissiondate
  on \@submissiondate{}
  \fi
  \ifcam@firstyr
  in partial fulfilment of the requirements
  \fi
  \ifcam@secondyr
  in partial fulfilment of the requirements
  \fi
  for the degree of Doctor of Philosophy%
}
\newcommand{\submissionnotice}[1]{\renewcommand{\@submissionnotice}{#1}}

% DOCVAR: collegeshield (The name of the file that contains the image of the
% college's shield. If `collegeshield' is provided, it will be included in the
% title page (just below the author's name and above the name of the college).
\newif\ifcam@collegeshield\cam@collegeshieldfalse
\newcommand{\@collegeshield}{}
\newcommand{\collegeshield}[1]{%
\renewcommand{\@collegeshield}{#1}\cam@collegeshieldtrue}

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%
%% Chapter and section numbering
%%
% TODO: Check whether numbering up to the fourth level is okay. It is not
% specified in the guidelines.
\setcounter{secnumdepth}{3}
\setcounter{tocdepth}{3}

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%
%% Front matter
%%
%% - outside and inside front cover
%% - title leaf
%% Do not include the date of make!
%% Institution + department.
%% Names of referees. (optional)
%% Degree.
%% Date of submission and defense. (optional)
%% Place and date of publication and publishers (and other info by them).
%%%%%
\newcommand{\frontmatter}{

  \pagestyle{empty}

  %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
  %%
  %% Title page components
  %%
  %%%%%
  \ifcam@techreport
  % Technical report mustn't have the custom title page (a standard one will be
  % prepended by the editor, see
  % http://www.cl.cam.ac.uk/techreports/submission.html).
  \else
  % The boxes below are all that will be displayed on the title page. They are
  % used to calculate exactly how much space should be left between them
  % (vertically).

  %% LOGO box
  \newlength{\cam@logorightnudge}
  \setlength{\cam@logorightnudge}{-0.5\paperwidth+12mm}
  \newsavebox{\cam@logo}
  \begin{lrbox}{\cam@logo}
    \hspace*{\cam@logorightnudge}
    %\includegraphics[width=73mm]{CollegeShields/CUni}
  \end{lrbox}

  %% THESIS TITLE box
  \newsavebox{\cam@title}
  \begin{lrbox}{\cam@title}
    \begin{minipage}[c][\height][c]{.98\textwidth}
      \begin{center}
        \Huge%
        \ifcam@times\else%
        \bfseries%
        \fi%
        {\@title{}}%
        \ifcam@firstyr\\%
        {\vspace{5mm}\emph{\LARGE PhD Proposal}}%
        \fi%
        \ifcam@secondyr\\%
        {\vspace{5mm}\emph{\LARGE Dissertation Schedule}}%
        \fi
      \end{center}
    \end{minipage}
  \end{lrbox}

  %% COLLEGESHIELD box (optional):
  \ifcam@collegeshield%
  \newsavebox{\cam@collegeshieldbox}
  \begin{lrbox}{\cam@collegeshieldbox}
    \includegraphics[height=20mm]{\@collegeshield}
  \end{lrbox}
  \fi

  %% AUTHOR&COLLEGE box
  \newsavebox{\cam@authorcollege}
  \begin{lrbox}{\cam@authorcollege}
    \begin{minipage}[c][\height][c]{.98\textwidth}
      \begin{center}
        {\large \@author{}~\\[1ex]}
        \ifcam@collegeshield%
        \vspace{2mm}{\usebox{\cam@collegeshieldbox}}\\
        \fi
        \@college{}
      \end{center}
    \end{minipage}
  \end{lrbox}

  %% SUBMISSION NOTICE box
  \newsavebox{\cam@submitnotice}
  \begin{lrbox}{\cam@submitnotice}
    \begin{minipage}[c][\height][c]{.98\textwidth}
      \begin{center}
        \@submissionnotice{}
      \end{center}
    \end{minipage}
  \end{lrbox}

  % Now calculate the exact free vertical space
  \newlength{\cam@titlepagevspace}
  \setlength{\cam@titlepagevspace}{\textheight%
    -\totalheightof{\usebox{\cam@logo}}%
    -\totalheightof{\usebox{\cam@submitnotice}}%
    -\totalheightof{\usebox{\cam@authorcollege}}%
  -\totalheightof{\usebox{\cam@title}}}

  %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
  %%
  %% Title Page: Put the components (logo, title, author, college and submit
  %% notice) onto the title page.
  %%
  %%%%%
  \begin{center}
    ~\vspace{.02\cam@titlepagevspace}\\
    {\usebox{\cam@logo}}\\
    \vspace{.28\cam@titlepagevspace}
    {\usebox{\cam@title}}\\
    \vspace{.23\cam@titlepagevspace}
    {\usebox{\cam@authorcollege}}\\
    \null\vfill
    {\usebox{\cam@submitnotice}}
  \end{center}
  \hypersetup{pdfsubject={\@subjectline},pdfkeywords={\@keywords}}
  \fi

  % Epigraph on odd page. (optional)

  %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
  %%
  %% Declaration
  %%
  %%%%%
  \ifcam@techreport
  % Technical report doesn't need the declaration
  % (see http://www.cl.cam.ac.uk/techreports/submission.html).
  \else
  {\ifcam@firstyr
    % First and second yr report don't need the declaration
    \else
    \ifcam@secondyr
    %
    \else
    \chapter*{Declaration}
    \thispagestyle{empty}
    This dissertation is the result of my own work and includes
    nothing which is the outcome
    of work done in collaboration except as declared in the Preface
    and specified in the text.
    It is not substantially the same as any that I have submitted, or
    am concurrently
    submitting, for a degree or diploma or other qualification at the
    University of Cambridge
    or any other University or similar institution except as declared
    in the Preface and
    specified in the text. I further state that no substantial part
    of my dissertation has
    already been submitted, or is being concurrently submitted, for
    any such degree, diploma
    or other qualification at the University of Cambridge or any
    other University or similar
    institution except as declared in the Preface and specified in the text.
    This dissertation does not exceed the prescribed limit of 60\,000 words.

    % Leaving some space for the signature:
    \vspace{15mm}

    \begin{flushright}
      \@author{}\\
      \@date{}\\
    \end{flushright}
    \vfill
    \fi
  \fi}
  \fi

  %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
  %%
  %% Abstract
  %%
  %%%%%
  \ifcam@techreport%
  \setcounter{page}{3}
  \fi

  \ifcam@firstyr
  % First yr report doesn't need a standalone abstract
  \else

  \chapter*{Abstract}
  \thispagestyle{empty}

  % Cambridge thesis submission guidelines require the title and
  % author be in the abstract.
% For more info see https://www.cambridgestudents.cam.ac.uk/your-course/examinations/graduate-exam-information/after-examination/degree-approval-and-1 % tex-fmt: skip
  \textbf{\large \@title}
  \par\vspace{0.3cm}
  \noindent\textit{\@author}
  \par\vspace{0.6cm}

  \@abstract{}
  \fi

  %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
  %%
  %% Acknowledgements
  %%
  %%%%%

  \ifcam@firstyr
  % First and second yr report don't need the acknowledgements
  \else
  {\ifcam@secondyr
    %
    \else
    \chapter*{Acknowledgements}
    \thispagestyle{empty}
    \@acknowledgements{}
  \fi}
  \fi

  %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
  %%
  %% Table of contents, figures, symbols and glossary.
  %%
  %%%%%

  % The following command prevents the page number to be displayed on the first
  % page of the TOC.
  \addtocontents{toc}{\protect\thispagestyle{empty}}
  \pagestyle{empty}
  \tableofcontents{}

  \ifcam@glossary%
  \cam@printthesisglossary
  \fi

  \cleardoublepage
  \pagestyle{plain}
}

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%
%% Backrefs
%%
%%%%%

\ifcam@backrefs
\RequirePackage[hyperpageref]{backref}
\renewcommand*{\backref}[1]{}
\renewcommand*{\backrefalt}[4]{%
  \ifcase #1 %
  \or
  {\footnotesize Cited on page #2.}%
  \else
  {\footnotesize Cited on pages #2.}%
  \fi
}
\fi

%%%%% EOF: cam-thesis.cls
