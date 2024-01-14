import Link from "next/link";

export default function Nav() {
  return (
    <nav>
      <div>
        <Link href="/" className="name">
          Jack Palaia<span className="cursor">|</span>
        </Link>
        {/* <Link href="/about" className="about">
          About
        </Link>
        <Link href="/projects" className="projects">
          Projects
        </Link>
        <Link href="/professional" className="professional">
          Professional
        </Link> */}
      </div>
    </nav>
  );
}
