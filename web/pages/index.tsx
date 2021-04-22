import Head from 'next/head';
import Link from 'next/link';
import dynamic from 'next/dynamic';
import fs from 'fs';
import {join} from 'path';

const App = dynamic(
  () => import('../components/app').then((mod) => mod.App),
  {
    ssr: false
  }
 );


function Page({mdText}) {
  return (
    <div>
      <Head>
        <link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/bootstrap/3.2.0/css/bootstrap.min.css" />
        <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.13.2/dist/katex.min.css"
          integrity="sha384-Cqd8ihRLum0CCg8rz0hYKPoLZ3uw+gES2rXQXycqnL5pgVQIflxAUDS7ZSjITLb5" crossOrigin="true"/>
        <link rel="icon" href="favicon.ico" />
        <title>yukicoder-md Playground</title>
      </Head>
      <nav className="navbar navbar-inverse navbar-static-top" role="navigation">
        <div className="container-fluid">
          <div className="navbar-header">
            <Link passHref href="https://koyumeishi.github.io/yukicoder-md/">
              <a className="navbar-brand">yukicoder-md</a>
            </Link>
          </div>

          <div className="collapse navbar-collapse" id="bs-example-navbar-collapse-1">
            <ul className="nav navbar-nav">
              <li><Link href="https://github.com/koyumeishi/yukicoder-md">GitHub</Link></li>
            </ul>
          </div>
        </div>
      </nav>
      <App mdText={mdText}/>
    </div>
  );
}

export async function getStaticProps(){
  const path = join(process.cwd(), 'resource', 'example.md');
  const mdText = fs.readFileSync(path, 'utf8');
  return {props: {mdText}};
}

export default Page;