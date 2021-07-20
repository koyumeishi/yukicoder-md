import React from "react";
import Link from "next/link";

import Split from 'react-split-grid';

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
		<div className="">
			<Split 
				render={({getGridProps, getGutterProps}) => (
					<div className="grid" {...getGridProps()}>
						<Left mdText={mdText}/>
						<div className="gutter-col gutter-col-1" {...getGutterProps('column', 1)} />
						<Right />
					</div>
				)}
			/>
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
		<div className="sg-left">
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
			    	<Link href="#" scroll={false}><a>Preview</a></Link>
			    </li>
			    <li onClick={()=>{setPreviewMode(false);}}
			    	className={previewMode ? "" : "active"}>
			    	<Link href="#" scroll={false}><a>HTML</a></Link>
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
		<div className="sg-right">
			{nav}
			{content}
		</div>
	);
};
