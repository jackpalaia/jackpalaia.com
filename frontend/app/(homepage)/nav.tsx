import Link from "next/link";
import GitHubIcon from "./icons/GitHubIcon";

export default function Nav() {
  return (
    <nav>
      <Link href="/" className="name">
        Jack Palaia
      </Link>
      <GitHubIcon />
    </nav>
  );
}
