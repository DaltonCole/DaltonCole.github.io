//! The hand-written personal pages.

use leptos::prelude::*;

const RESUME: &str = "/static/home/pdf/resume.pdf";
const PROFILE_PHOTO: &str = "/static/home/images/profile_pic.jpg";
const CAGA_POSTER: &str = "/static/home/pdf/TITANS_CCD2018_CAGA.PDF";
const FROGEYE_POSTER: &str = "/static/home/pdf/FrogEye_CCD_2017.PDF";
const CONAN_POSTER: &str = "/static/home/pdf/Conan_CCD_2017.PDF";
const FACE_AUTH_PAPER: &str = "/static/research/papers/attack_face_authentication.pdf";
const FACE_AUTH_DOI: &str = "https://doi.org/10.1109/TDSC.2021.3067794";
const FACE_AUTH_WRITEUP: &str =
    "https://blog.chameleoncloud.org/posts/a-new-facial-authentication-pitfall-and-remedy-in-web-service/";
const DISSERTATION: &str =
    "https://mospace.umsystem.edu/items/7a46c56e-ef97-4b60-aa9a-6b1e90ec3a71";
const TEACHING_PHOTO: &str = "/static/home/images/dalton_is_the_best.jpg";
const LEADERSHIP_AWARD_PHOTO: &str = "/static/home/images/leadership_award.jpg";
const SIG_COMP_WIN_PHOTO: &str = "/static/home/images/sig_comp.jpg";
const SIG_COMP_GROUP_PHOTO: &str = "/static/home/images/sig_comp_group.jpg";
const SIG_SEC_PHOTO: &str = "/static/home/images/sig_sec_me.jpg";
const ASUM_PHOTO: &str = "/static/home/images/clair_mccaskill.jpg";

#[component]
fn Ext(#[prop(into)] href: String, children: Children) -> impl IntoView {
    view! { <a href=href target="_blank" rel="noopener">{children()}</a> }
}

#[component]
fn Card(
    #[prop(into)] href: String,
    #[prop(into)] title: String,
    accent: &'static str,
    #[prop(optional)] icon: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let style =
        format!("--card-accent: var(--{accent}); --card-accent-soft: var(--{accent}-soft);");
    view! {
        <a href=href class="card" style=style>
            {icon.map(|letter| view! { <div class="card-icon">{letter}</div> })}
            <h3>{title}</h3>
            {children()}
        </a>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <section class="hero">
            <div class="container">
                <div class="eyebrow hero-tag">"Senior Computer Scientist · Ph.D., Computer Science"</div>
                <h1>"Hi, I'm Dalton Cole."</h1>
                <p class="lede">
                    "I'm a Senior Computer Scientist at Sandia National Laboratories, where I work "
                    "on machine learning and remote sensing. I hold a Ph.D. in Computer Science "
                    "from the University of Missouri, with a research focus on adversarial "
                    "robustness and security in ML systems."
                </p>
                <div class="button-row">
                    <a class="btn btn-primary" href="/contact/">"Get in touch"</a>
                    <a class="btn btn-secondary" href="/resume/">"View my résumé"</a>
                </div>
            </div>
        </section>

        <section>
            <div class="container">
                <div class="eyebrow" style="color: var(--violet)">"Explore"</div>
                <h2>"Start here"</h2>
                <div class="card-grid">
                    <Card href="/about/" title="About" accent="violet" icon="A">
                        <p>"Who I am, and the path that got me here — a career timeline from high school to Sandia."</p>
                    </Card>
                    <Card href="/work-experience/" title="Experience" accent="amber" icon="E">
                        <p>"Research and teaching roles, publications, leadership, and the projects behind them."</p>
                    </Card>
                    <Card href="/projects/" title="Projects" accent="pink" icon="P">
                        <p>"Interactive tools written in Rust, like a Phase 10 score keeper."</p>
                    </Card>
                </div>
            </div>
        </section>
    }
}

#[component]
fn TimelineItem(
    date: &'static str,
    title: &'static str,
    accent: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="timeline-item" style=format!("--item-accent: var(--{accent})")>
            <div class="timeline-date">{date}</div>
            <h3>{title}</h3>
            <p>{children()}</p>
        </div>
    }
}

#[component]
pub fn About() -> impl IntoView {
    view! {
        <section>
            <div class="container about-intro">
                <img class="profile-photo" src=PROFILE_PHOTO alt="Dalton Cole"/>
                <div>
                    <div class="eyebrow" style="color: var(--violet)">"About"</div>
                    <h1>"Dalton Russell Cole"</h1>
                    <p>
                        "Also known as Dr. Cole, Daltie, Coltie, and a handful of other names that "
                        "stuck over the years. I'm a Senior Computer Scientist at Sandia National "
                        "Laboratories, working on machine learning and remote sensing — and I'm "
                        "glad you stopped by."
                    </p>
                </div>
            </div>
        </section>

        <section>
            <div class="container">
                <h2>"Timeline"</h2>
                <div class="timeline">
                    <TimelineItem date="July 2021 — Present" title="Senior Computer Scientist at Sandia National Laboratories" accent="pink">
                        "Joined SNL (not the comedy show) full-time. I support the development and "
                        "analysis of wide-view, visual-based satellite remote sensing sensors, lead "
                        "a team of engineers analyzing remote sensing data, design graph neural "
                        "networks for event classification, and mentor Ph.D. and undergraduate interns."
                    </TimelineItem>
                    <TimelineItem date="July 2021" title="Ph.D. in Computer Science" accent="violet">
                        "Graduated with my Ph.D. in Computer Science from "
                        <Ext href="https://missouri.edu/">"Mizzou"</Ext>
                        "."
                    </TimelineItem>
                    <TimelineItem date="Spring 2020" title="Database GTA" accent="cyan">
                        "Graduate teaching assistant for CS3380, Database Applications and "
                        "Information Systems at Mizzou — relational algebra, SQL, and grading a "
                        "lot of assignments."
                    </TimelineItem>
                    <TimelineItem date="Fall 2019" title="Taught Introduction to Programming" accent="amber">
                        "Taught CS1570 at Missouri S&T, an introductory C++03 course covering "
                        "everything from data types to templated classes, plus writing homework "
                        "and exams."
                    </TimelineItem>
                    <TimelineItem date="May 2019" title="Master's Degree" accent="pink">
                        "Graduated with a Master's degree in Computer Science from "
                        <Ext href="https://www.mst.edu/">"Missouri S&T"</Ext>
                        " with a 3.92/4.0 GPA."
                    </TimelineItem>
                    <TimelineItem date="May 2017 – Dec. 2019" title="Cybersecurity Intern at Sandia National Laboratories" accent="violet">
                        "Worked across 7 different projects, ranging from the 1553 data bus "
                        "protocol on aerial vehicles to machine learning and data analytics."
                    </TimelineItem>
                    <TimelineItem date="Summer 2016" title="Intern at Cerner" accent="cyan">
                        "Built a Ruby on Rails app to streamline my team's development process, "
                        "integrating with Jenkins, Crucible, and Jira on top of MongoDB."
                    </TimelineItem>
                    <TimelineItem date="May 2016" title="Bachelor's Degree" accent="amber">
                        "Graduated with a B.S. in Computer Science from "
                        <Ext href="https://www.mst.edu/">"Missouri S&T"</Ext>
                        ", with minors in Computer Engineering and Math, 3.83/4.0 GPA."
                    </TimelineItem>
                    <TimelineItem date="May 2013" title="Graduated High School" accent="pink">
                        "Graduated from "
                        <Ext href="https://www.nkcschools.org/Domain/38">"Winnetonka High School"</Ext>
                        " with the Gold Medallion Honors Diploma."
                    </TimelineItem>
                </div>
            </div>
        </section>
    }
}

#[component]
fn SubSection(#[prop(into)] title: String, children: Children) -> impl IntoView {
    view! {
        <div class="exp-sub">
            <h4>{title}</h4>
            {children()}
        </div>
    }
}

#[component]
fn Poster(title: &'static str, href: &'static str) -> impl IntoView {
    view! {
        <SubSection title=title>
            <p><Ext href=href>"Project poster"</Ext></p>
        </SubSection>
    }
}

/// `width` and `height` are the file's real size. They let the browser reserve the space before a
/// lazy photo loads, so the side menu's jump links land in the right place.
#[component]
fn Photo(
    src: &'static str,
    alt: &'static str,
    width: u32,
    height: u32,
    #[prop(optional)] narrow: bool,
) -> impl IntoView {
    let class = if narrow {
        "exp-photo exp-photo-narrow"
    } else {
        "exp-photo"
    };
    view! { <img class=class src=src alt=alt width=width height=height loading="lazy"/> }
}

#[component]
fn Role(
    id: &'static str,
    when: &'static str,
    accent: &'static str,
    title: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div id=id class="exp-section">
            <div class="exp-role" style=format!("color: var(--{accent})")>{when}</div>
            <h2>{title}</h2>
            {children()}
        </div>
    }
}

#[component]
pub fn Experience() -> impl IntoView {
    let side_links = [
        ("#sandia", "Sandia"),
        ("#publications", "Publications"),
        ("#research", "Research Projects"),
        ("#database-gta", "Database GTA"),
        ("#teaching", "Teaching"),
        ("#cerner", "Cerner"),
        ("#acm", "ACM"),
        ("#asum", "ASUM"),
        ("#other", "Other"),
    ]
    .into_iter()
    .map(|(href, label)| view! { <a href=href>{label}</a> })
    .collect_view();

    view! {
        <section>
            <div class="container">
                <div class="eyebrow" style="color: var(--amber)">"Experience"</div>
                <h1>"Work & Leadership"</h1>
                <p class="lede">
                    "Research, publications, teaching, and organizational leadership from Sandia "
                    "National Laboratories back through undergrad."
                </p>
                <div class="button-row">
                    <a class="btn btn-primary" href=RESUME target="_blank" rel="noopener">"Download résumé (PDF)"</a>
                </div>
            </div>
        </section>

        <section>
            <div class="container experience-layout">
                <nav class="experience-side-nav">{side_links}</nav>
                <div>
                    <Role id="sandia" when="May 2017 – Present" accent="amber" title="Sandia National Laboratories">
                        <p>
                            <Ext href="https://www.sandia.gov/">"Sandia National Laboratories"</Ext>
                            " — first as a year-round intern from May 2017 to December 2019, then "
                            "full-time since July 2021. Safety and security are the top priorities "
                            "there, but the day-to-day culture is refreshingly relaxed."
                        </p>
                        <p>"I've worked on roughly 7 projects across my time at Sandia. I have permission to discuss three:"</p>
                        <Poster title="CAGA: Counter Adversarial Graph Analytics" href=CAGA_POSTER/>
                        <Poster title="FrogEye: Joint Analysis of Source Code and Binaries Using Machine Learning" href=FROGEYE_POSTER/>
                        <Poster title="Conan: Emulytics on the 1553 Protocol" href=CONAN_POSTER/>
                        <SubSection title="Other">
                            <p>"Data analytics, building Python modules in C++, and software analysis."</p>
                        </SubSection>
                    </Role>

                    <Role id="publications" when="2021 – 2022" accent="pink" title="Publications">
                        <p>
                            <strong>"Dalton Cole"</strong>
                            ", Sara Newman, Dan Lin. "
                            <Ext href=FACE_AUTH_PAPER>"A New Facial Authentication Pitfall and Remedy in Web Services"</Ext>
                            ". "
                            <em>"IEEE Transactions on Dependable and Secure Computing"</em>
                            ", vol. 19, no. 4, pp. 2635–2647, July–Aug. 2022. "
                            <Ext href=FACE_AUTH_DOI>"DOI"</Ext>
                            "."
                        </p>
                        <p>
                            <strong>"Dalton Russell Cole"</strong>
                            ". "
                            <Ext href=DISSERTATION>"Defeat Data Poisoning Attacks on Facial Recognition Applications"</Ext>
                            ". Ph.D. dissertation, University of Missouri–Columbia, 2021."
                        </p>
                    </Role>

                    <Role id="research" when="2017 – 2021" accent="cyan" title="Research Projects">
                        <p>
                            "Graduate research on data poisoning attacks against facial recognition at "
                            "the University of Missouri, and on grammars and malware analysis at "
                            "Missouri S&T."
                        </p>
                        <SubSection title="Deepfake Detector Attack and Defense — 2021">
                            <p>
                                "Designed a label-flipping data poisoning attack on deepfake detectors, "
                                "evaluated against XceptionNet in PyTorch, that exceeded a 99% poison "
                                "success rate in most cases. As a defense, built a convolutional neural "
                                "network that spots poisoned images, with a detection rate above 98% and "
                                "precision above 99% in most cases."
                            </p>
                            <p><Ext href=DISSERTATION>"Dissertation"</Ext></p>
                        </SubSection>
                        <SubSection title="Facial Authentication Attack and Defense — 2020">
                            <p>
                                "Showed that slipping a handful of an attacker's photos into a web "
                                "service's face enrollment, for example through a compromised home "
                                "router, lets the attacker log in as the victim with no knowledge of "
                                "the server. Against FaceNet on the FEI and LFW datasets, the attacker "
                                "passed a 90% recognition rate once their photos made up half of the "
                                "enrollment set. The attack also worked against BioID, a real face "
                                "authentication app."
                            </p>
                            <p>
                                "Built DEFEAT, a deep learning detector that flags poisoned enrollments. "
                                "It stayed above 90% detection accuracy in every scenario tested, and "
                                "passed 99% as injected photos neared half of the enrollment set."
                            </p>
                            <p>
                                <Ext href=FACE_AUTH_PAPER>"Paper"</Ext>
                                " · "
                                <Ext href=FACE_AUTH_WRITEUP>"Project write-up"</Ext>
                            </p>
                        </SubSection>
                        <SubSection title="Grammar Induction — 2019">
                            <ul>
                                <li>
                                    <Ext href="https://github.com/DaltonCole/Grammar-Malware-Analysis">"Grammar Malware Analysis"</Ext>
                                    ": used the ANTLR parser generator to compare how JavaScript grammar "
                                    "rules are used in benign code and in malware, with a Scrapy web "
                                    "crawler gathering the benign samples."
                                </li>
                                <li>
                                    <Ext href="https://github.com/DaltonCole/research_ea">"Evolutionary Algorithm Based Grammar Induction"</Ext>
                                    ": a C++ evolutionary algorithm that generates simple context-free "
                                    "grammars from samples of a language."
                                </li>
                            </ul>
                        </SubSection>
                    </Role>

                    <Role id="database-gta" when="Spring 2020" accent="pink" title="Database Applications GTA">
                        <p>
                            "Graduate teaching assistant for CMP_SC 3380, Database Applications and "
                            "Information Systems at Mizzou — office hours, grading, and covering the "
                            "fundamentals of database management systems, data modeling, relational "
                            "algebra, and SQL."
                        </p>
                    </Role>

                    <Role id="teaching" when="Fall 2019" accent="violet" title="Introduction to Programming GTA">
                        <Photo src=TEACHING_PHOTO alt="Dalton Is The Best Teaching Photo" width=320 height=895 narrow=true/>
                        <p>
                            "Graduate teaching assistant at "
                            <Ext href="https://www.mst.edu/">"Missouri University of Science & Technology"</Ext>
                            ", responsible for two sections of CS 1570, Introduction to Programming — "
                            "C++03 for students with little to no prior programming experience. I also "
                            "wrote exams, helped write assignments, and held weekly office hours."
                        </p>
                        <p>"Topics covered, roughly least to most difficult:"</p>
                        <ul>
                            <li>"Systems, software, compilers, and algorithms"</li>
                            <li>"Variable types, operators, control flow, structs, file I/O, arrays"</li>
                            <li>"Pass-by-value vs. pass-by-reference, the run-time stack, function overloading, static variables"</li>
                            <li>"Object-oriented programming: classes, access control, friend functions, constructors, templated classes"</li>
                            <li>"Namespaces, pointers, and — drilled in relentlessly — documentation"</li>
                        </ul>
                    </Role>

                    <Role id="cerner" when="Summer 2016" accent="cyan" title="Cerner Intern">
                        <p>
                            "Software engineering intern at "
                            <Ext href="https://www.cerner.com/">"Cerner"</Ext>
                            ", leading a team of three interns building a Ruby on Rails app to "
                            "streamline our teams' development process. Integrated APIs from "
                            "Jenkins, Crucible, and Jira (and scraped what didn't have one), backed "
                            "by MongoDB. Added input validation and other security hardening after "
                            "teammates went looking for holes in the login flow — and found time to "
                            "sneak in a live panda-cam Easter egg. 🐼"
                        </p>
                    </Role>

                    <Role id="acm" when="2013 – 2017" accent="amber" title="Association for Computing Machinery">
                        <p>
                            <Ext href="https://www.mstacm.org/">"ACM"</Ext>
                            " at Missouri S&T ran three Special Interest Groups under a general body: "
                            <Ext href="https://sigcomp.github.io/">"SIG-Comp"</Ext>
                            " (competitive programming), "
                            <Ext href="https://acmsigsec.mst.edu/">"SIG-Sec"</Ext>
                            " (CTFs and security workshops), and "
                            <Ext href="https://siggame.io/">"SIG-Game"</Ext>
                            " (a new game every semester, capped by the 24-hour MegaMinerAI contest)."
                        </p>
                        <SubSection title="SIG-Competition Chair — 2016–17">
                            <p>
                                "Planned the local 5-hour ACM Programming Cup and gave algorithm "
                                "lectures when our advisor wasn't available."
                            </p>
                            <ul>
                                <li>
                                    "First place on-site at two "
                                    <Ext href="https://icpc.global/">"ICPC"</Ext>
                                    " competitions at Webster University (2015, 2017)"
                                </li>
                                <li>"First place individual at MS&T's ACM Programming Cup (Fall 2019)"</li>
                            </ul>
                            <div class="exp-photo-pair">
                                <Photo src=SIG_COMP_WIN_PHOTO alt="SIG-COMP First Place on Site - Webster University" width=800 height=360/>
                                <Photo src=SIG_COMP_GROUP_PHOTO alt="SIG-COMP Group Photo" width=800 height=438/>
                            </div>
                        </SubSection>
                        <SubSection title="Secretary — 2014–15, 2016–17">
                            <p>"Advertised ACM General events and took officer meeting notes."</p>
                        </SubSection>
                        <SubSection title="President — 2015–16">
                            <Photo src=LEADERSHIP_AWARD_PHOTO alt="Leadership Award" width=300 height=408 narrow=true/>
                            <p>
                                "Found sponsors, led meetings, approved budgets in the thousands, "
                                "equipped every sub-org (including laptops for the Cyber Defense "
                                "team), and scheduled company tech talks. Voted the Computer Science "
                                "Leadership Award by my peers that year."
                            </p>
                        </SubSection>
                        <SubSection title="SIG-Security — Member, Cyber Defense Team (M57)">
                            <Photo src=SIG_SEC_PHOTO alt="Rare image of competing in a CTF competition with the sun up" width=800 height=450/>
                            <p>
                                "Took part in the first four Sandia-partnered cybersecurity "
                                "bootcamps in Albuquerque — Metasploit, "
                                <Ext href="https://www.hex-rays.com/products/ida/">"IDA Pro"</Ext>
                                " reverse engineering, and network mapping with nmap and Netmeld — "
                                "plus late nights competing in "
                                <Ext href="https://ctftime.org/ctf-wtf/">"CTF"</Ext>
                                " competitions as part of M57."
                            </p>
                        </SubSection>
                    </Role>

                    <Role id="asum" when="2013 – 2015" accent="violet" title="Associated Students of the University of Missouri">
                        <Photo src=ASUM_PHOTO alt="ASUM with Claire McCaskill" width=958 height=743/>
                        <p>
                            "ASUM worked to get students more involved in politics and lobbied on "
                            "their behalf — including two trips to Washington, D.C. to talk with the "
                            "House and Senate on Missouri's behalf. As programming coordinator "
                            "(2014–15) I planned events like \"pie and politics,\" trivia and bowling "
                            "nights, an annual screening of V for Vendetta, and proposed our budget "
                            "to ResLife."
                        </p>
                    </Role>

                    <Role id="other" when="2012 – 2015" accent="cyan" title="Other Organizations">
                        <SubSection title="Downtown Hall Association — Vice President">
                            <p>"Led meetings in the president's absence, guided hall governors on hall projects, and was elected St. Pat Knight for DHA."</p>
                        </SubSection>
                        <SubSection title="History Club — Vice President">
                            <p>"Mostly an excuse for free pizza, good friends, and talking history."</p>
                        </SubSection>
                        <SubSection title="Gold Medallion">
                            <p>
                                "Missouri's honors diploma, requiring strong grades, 100 hours of "
                                "community service (earning the Presidential Service Award too), and "
                                "a dissertation-style research project — mine was on online/e-learning, "
                                "including three interviews with professionals in the field."
                            </p>
                        </SubSection>
                        <SubSection title="Assisteens — President">
                            <p>
                                "Led our local chapter of "
                                <Ext href="https://www.assistanceleague.org/kansas-city/volunteer/assisteens/">"Assisteens"</Ext>
                                ", a youth branch of the Assistance League of Kansas City, during the "
                                "2012–13 school year."
                            </p>
                        </SubSection>
                    </Role>
                </div>
            </div>
        </section>
    }
}

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <section>
            <div class="container">
                <div class="eyebrow" style="color: var(--cyan)">"Contact"</div>
                <h1>"Let's talk."</h1>
                <p class="lede">"Question, opportunity, or just want to say hi? Here's where to find me."</p>
                <div class="contact-methods">
                    <a class="contact-method" href="mailto:contact@daltoncole.com">
                        <span class="dot"></span>
                        <span class="contact-label">"Email"</span>
                        <span>"contact@daltoncole.com"</span>
                    </a>
                    <a class="contact-method" href="https://github.com/DaltonCole" target="_blank" rel="noopener">
                        <span class="dot"></span>
                        <span class="contact-label">"GitHub"</span>
                        <span>"DaltonCole"</span>
                    </a>
                    <a class="contact-method" href="https://www.linkedin.com/in/daltoncole1/" target="_blank" rel="noopener">
                        <span class="dot"></span>
                        <span class="contact-label">"LinkedIn"</span>
                        <span>"daltoncole1"</span>
                    </a>
                </div>
            </div>
        </section>
    }
}

#[component]
pub fn Resume() -> impl IntoView {
    view! {
        <section class="doc">
            <div class="container wide">
                <div class="eyebrow" style="color: var(--amber)">"Résumé"</div>
                <h1 class="doc-title">"Résumé"</h1>
                <p class="doc-meta">"Also check out my " <a href="/work-experience/">"Work Experience"</a> " page!"</p>
                <object class="resume-frame" data=RESUME type="application/pdf">
                    <p>"Oh no! You don't have a PDF plugin for this browser. To download this PDF click " <a href=RESUME>"HERE"</a> "."</p>
                </object>
            </div>
        </section>
    }
}
