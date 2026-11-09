//! The hand-written personal pages.

use leptos::prelude::*;

const RESUME: &str = "/static/home/pdf/resume.pdf";
const PROFILE_PHOTO: &str = "/static/home/images/profile_pic.jpg";
const CAGA_POSTER: &str = "/static/home/pdf/TITANS_CCD2018_CAGA.PDF";
const FROGEYE_POSTER: &str = "/static/home/pdf/FrogEye_CCD_2017.PDF";
const CONAN_POSTER: &str = "/static/home/pdf/Conan_CCD_2017.PDF";

/// Set to a Formspree endpoint (`https://formspree.io/f/xxxxxxx`) to show the contact form.
/// It stays hidden until then, because a form with no endpoint would lose people's messages.
const CONTACT_FORM_ENDPOINT: Option<&str> = None;

#[component]
fn Ext(#[prop(into)] href: String, children: Children) -> impl IntoView {
    view! { <a href=href target="_blank" rel="noopener">{children()}</a> }
}

#[component]
pub fn Card(
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
                <div class="eyebrow hero-tag">"Software Engineer · Ph.D., Computer Science"</div>
                <h1>"Hi, I'm Dalton Cole."</h1>
                <p class="lede">
                    "I'm a research scientist at Sandia National Laboratories, where I work on "
                    "automation and machine learning. I hold a Ph.D. in Computer Science from the "
                    "University of Missouri, with a research focus on adversarial robustness and "
                    "security in ML systems."
                </p>
                <div class="button-row">
                    <a class="btn btn-primary" href="/contact/">"Get in touch"</a>
                    <a class="btn btn-secondary" href="/work-experience/">"See my experience"</a>
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
                        <p>"Research and teaching roles, leadership, and the projects behind them."</p>
                    </Card>
                    <Card href="/contact/" title="Contact" accent="cyan" icon="C">
                        <p>"Have a question or an opportunity? Reach out — I'd love to hear from you."</p>
                    </Card>
                </div>
            </div>
        </section>

        <section>
            <div class="container">
                <div class="eyebrow" style="color: var(--pink)">"Dig in"</div>
                <h2>"More on the site"</h2>
                <div class="card-grid">
                    <Card href="/projects/" title="Projects" accent="pink">
                        <p>"Small tools and experiments, like a Phase 10 score keeper."</p>
                    </Card>
                    <Card href="/research/" title="Research" accent="violet">
                        <p>"Papers, projects, and notes on adversarial machine learning."</p>
                    </Card>
                    <Card href="/writeups/" title="Write-ups" accent="amber">
                        <p>"Solutions and explanations for Kattis competitive programming problems."</p>
                    </Card>
                    <Card href="/learn/" title="Learn" accent="cyan">
                        <p>"Notes on topics I'm studying, organized by theme and subject."</p>
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
                        "stuck over the years. I'm a software engineer whose research interests "
                        "center on automation and machine learning — and I'm glad you stopped by."
                    </p>
                </div>
            </div>
        </section>

        <section>
            <div class="container">
                <h2>"Timeline"</h2>
                <div class="timeline">
                    <TimelineItem date="July 2021 — Present" title="Sandia National Laboratories" accent="pink">
                        "Joined SNL (not the comedy show) to work on automation and machine learning research full-time."
                    </TimelineItem>
                    <TimelineItem date="June 2021" title="Ph.D. in Computer Science" accent="violet">
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
                    <TimelineItem date="May 2017 – Dec. 2019" title="Intern at Sandia National Laboratories" accent="violet">
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
        ("#teaching", "Teaching"),
        ("#cerner", "Cerner"),
        ("#database-gta", "Database GTA"),
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
                    "Research, teaching, and organizational leadership from Sandia National "
                    "Laboratories back through undergrad."
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

                    <Role id="teaching" when="Fall 2019" accent="violet" title="Introduction to Programming GTA">
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

                    <Role id="database-gta" when="Spring 2020" accent="pink" title="Database Applications GTA">
                        <p>
                            "Graduate teaching assistant for CMP_SC 3380, Database Applications and "
                            "Information Systems at Mizzou — office hours, grading, and covering the "
                            "fundamentals of database management systems, data modeling, relational "
                            "algebra, and SQL."
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
                        <SubSection title="President — 2015–16">
                            <p>
                                "Found sponsors, led meetings, approved budgets in the thousands, "
                                "equipped every sub-org (including laptops for the Cyber Defense "
                                "team), and scheduled company tech talks. Voted the Computer Science "
                                "Leadership Award by my peers that year."
                            </p>
                        </SubSection>
                        <SubSection title="Secretary — 2014–15, 2016–17">
                            <p>"Advertised ACM General events and took officer meeting notes."</p>
                        </SubSection>
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
                        </SubSection>
                        <SubSection title="SIG-Security — Member, Cyber Defense Team (M57)">
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
    let lede = if CONTACT_FORM_ENDPOINT.is_some() {
        "Question, opportunity, or just want to say hi? Send a message below, or reach out directly."
    } else {
        "Question, opportunity, or just want to say hi? Reach out directly."
    };
    let layout = if CONTACT_FORM_ENDPOINT.is_some() {
        "container contact-grid"
    } else {
        "container"
    };

    let form = CONTACT_FORM_ENDPOINT.map(|action| {
        view! {
            <div>
                <h2>"Send a message"</h2>
                <form action=action method="POST">
                    <div class="field">
                        <label for="name">"Name"</label>
                        <input id="name" name="name" type="text" required/>
                    </div>
                    <div class="field">
                        <label for="email">"Email"</label>
                        <input id="email" name="email" type="email" required/>
                    </div>
                    <div class="field">
                        <label for="message">"Message"</label>
                        <textarea id="message" name="message" rows="6" required></textarea>
                    </div>
                    <button class="btn btn-primary" type="submit">"Send message"</button>
                    <p class="form-note">"Sent via Formspree — replies go straight to my inbox."</p>
                </form>
            </div>
        }
    });

    view! {
        <section>
            <div class="container">
                <div class="eyebrow" style="color: var(--cyan)">"Contact"</div>
                <h1>"Let's talk."</h1>
                <p class="lede">{lede}</p>
            </div>
        </section>

        <section>
            <div class=layout>
                <div>
                    <h2>"Ways to reach me"</h2>
                    <div class="contact-methods">
                        <a class="contact-method" href="mailto:contact@daltoncole.com">
                            <span class="dot"></span>
                            <span>"contact@daltoncole.com"</span>
                        </a>
                        <a class="contact-method" href="https://github.com/DaltonCole" target="_blank" rel="noopener">
                            <span class="dot"></span>
                            <span>"GitHub — DaltonCole"</span>
                        </a>
                        <a class="contact-method" href="https://www.linkedin.com/in/daltoncole1/" target="_blank" rel="noopener">
                            <span class="dot"></span>
                            <span>"LinkedIn — daltoncole1"</span>
                        </a>
                    </div>
                </div>
                {form}
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
