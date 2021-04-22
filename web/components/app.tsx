import React from "react";
import Link from "next/link";

import {Editor, UseTemplateEngine} from './editor';
import {Editor as DomEditor} from './dom-display';
import {Display} from './display';

import {
	atom,
	useRecoilState,
} from 'recoil';



const PreviewMode = atom<boolean>({
	key: 'PreviewMode',
	default: true,
});


interface Props {
	mdText: string,
};


export const App: React.FC<Props> = ({mdText}) => {
	return (
		<div className="container-fluid">
			<div className="row">
				<Left mdText={mdText}/>
				<Right />
			</div>
		</div>
	);
};

const TemplateEngineCheckBox: React.FC<{}> = () => {
	const [useTemplateEngine, setUseTemplateEngine] = useRecoilState(UseTemplateEngine);
	return (
		<div className="checkbox">
		    <label>
			    <input
			        type="checkbox"
					checked={useTemplateEngine}
					onChange={() => setUseTemplateEngine(!useTemplateEngine)}/>
				Enable Template Engine (tera)
		    </label>
		</div>
	);
};


const Left: React.FC<Props> = ({mdText}) => {
	return (
		<div className="col-md-6">
			<TemplateEngineCheckBox />
			<Editor mdText={mdText} />
		</div>
	);
};


const Right: React.FC<{}> = () => {
	const [previewMode, setPreviewMode] = useRecoilState(PreviewMode);

	const nav = (
		<div>
			<ul className="nav nav-pills">
			    <li onClick={()=>{setPreviewMode(true);}}
			    	className={previewMode ? "active" : ""}>
			    	<Link href="#" scroll={false}>Preview</Link>
			    </li>
			    <li onClick={()=>{setPreviewMode(false);}}
			    	className={previewMode ? "" : "active"}>
			    	<Link href="#" scroll={false}>HTML</Link>
			    </li>
			</ul>
		</div>
	);
	const content = (
		<div style={{height: "80vh", overflow: "auto"}}>
			<div style={{display: previewMode ? "" : "none"}}>
				<Display />
			</div>
			<div style={{display: previewMode ? "none" : "", height: "inherit"}}>
				<DomEditor />
			</div>
		</div>
	);
	return (
		<div className="col-md-6">
			{nav}
			{content}
		</div>
	);
};
