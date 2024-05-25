%! TeX root = WGUnderwood.tex

% class
\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{wgu-cv}

% packages
\LoadClass[10pt]{article}
\RequirePackage[margin=1in,top=0.9in]{geometry}
\RequirePackage{hyperref}
%\RequirePackage{fontspec}
\RequirePackage{microtype}
\RequirePackage{fancyhdr}
\RequirePackage{enumitem}
\RequirePackage{ifthen}

% variables
\def\yourname#1{\def\@yourname{#1}}
\def\youraddress#1{\def\@youraddress{#1}}
\def\youremail#1{\def\@youremail{#1}}
\def\yourwebsite#1{\def\@yourwebsite{#1}}

% settings
%\setmainfont{Libre Baskerville}[Scale=0.9]
%\setmonofont{Source Code Pro}[Scale=0.97]
\geometry{a4paper}
\setlength\parindent{0pt}
\bibliographystyle{abbrvnat}
\pagestyle{fancy}
\renewcommand{\headrulewidth}{0pt}
\cfoot{\thepage}
\rfoot{\today}
\setlist{
  leftmargin=0.5cm,
  topsep=0cm,
  partopsep=0cm,
  parsep=-0.04cm, % item spacing
  before=\vspace{0.12cm},
  after=\vspace{0.08cm},
}

% arxiv
\newcommand{\arxiv}[1]{%
  \href{https://arxiv.org/abs/#1}{%
  \texttt{arXiv{:}{\allowbreak}#1}}%
}

% github
\newcommand{\github}[1]{%
  GitHub: \href{https://github.com/#1}{%
  \texttt{#1}}%
}

% title
\renewcommand{\maketitle}{%
  \vspace*{-1.2cm}%
  \begin{center}%
    \begin{huge}%
      \@yourname \\
    \end{huge}%
    \vspace{0.5cm}%
    \@youraddress \\
    \vspace{0.16cm}%
    \begin{minipage}{0.45\textwidth}%
      \centering%
      \href{mailto:\@youremail}{\nolinkurl{\@youremail}}%
    \end{minipage}%
    \begin{minipage}{0.45\textwidth}%
      \centering%
      \href{https://\@yourwebsite}{\nolinkurl{\@yourwebsite}}%
    \end{minipage}
  \end{center}%
}

% section
\renewcommand{\section}[1]{%
  \vspace{0.3cm}%
  \par\hbox{\large\textbf{#1}\strut}%
  \vspace{-0.25cm}%
  \rule{\textwidth}{0.8pt}%
  \vspace{-0.15cm}%
}

% subsection
\renewcommand{\subsection}[2]{%
  \vspace{0.30cm}%
  \textbf{#1}%
  \hfill{#2}%
  \vspace{0.03cm}%
}

% subsubsection
\renewcommand{\subsubsection}[1]{%
  \linebreak
  \textit{#1}%
  \vspace{0.05cm}%
}
