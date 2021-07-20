// import App from "next/app";
import {RecoilRoot} from 'recoil';
import type { AppProps , /*AppContext*/  } from 'next/app';

import '../styles/base.css';
import '../styles/problems.show.css';
import '../styles/splitgrid.css';

const MyApp = ({ Component, pageProps }: AppProps)  => {
  return (
    <RecoilRoot>
      <Component {...pageProps} />
    </RecoilRoot>
  );
}

// Only uncomment this method if you have blocking data requirements for
// every single page in your application. This disables the ability to
// perform automatic static optimization, causing every page in your app to
// be server-side rendered.
//
// MyApp.getInitialProps = async (appContext: AppContext) => {
//   // calls page's `getInitialProps` and fills `appProps.pageProps`
//   const appProps = await App.getInitialProps(appContext);

//   return { ...appProps }
// }

export default MyApp;
